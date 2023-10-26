use serde::{Deserialize, Serialize};

macro eval_once($ty:path, $expr:expr) {{
    static CACHE: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| $expr);

    &*CACHE
}}

#[derive(Serialize, Deserialize)]
#[serde(tag = "sub", rename_all = "camelCase")]
pub enum Token {
    Refresh {
        #[serde(flatten)]
        inner: Claims,
    },
    Session {
        #[serde(flatten)]
        inner: Claims,
    },
}

impl Token {
    pub fn sub(&self) -> &'static str {
        match self {
            Self::Refresh { .. } => "refresh",
            Self::Session { .. } => "session",
        }
    }

    pub fn is_refresh(&self) -> bool { matches!(self, Self::Refresh { .. }) }

    pub fn is_session(&self) -> bool { matches!(self, Self::Session { .. }) }
}

impl Token {
    pub fn encode(&self) -> anyhow::Result<String> {
        use jsonwebtoken as jwt;

        let key = eval_once!(jwt::EncodingKey, {
            use base64::prelude::{Engine, BASE64_STANDARD as engine};

            let raw = engine.decode(*crate::vars::JWT_ENC_KEY).unwrap();
            jwt::EncodingKey::from_ed_der(&raw)
        });

        let header = eval_once!(jwt::Header, {
            jwt::Header::new(jwt::Algorithm::EdDSA) //
        });

        Ok(jwt::encode(header, self, key)?)
    }

    pub fn decode(token: &str) -> anyhow::Result<Self> {
        use jsonwebtoken as jwt;

        let key = eval_once!(jwt::DecodingKey, {
            use base64::prelude::{Engine, BASE64_STANDARD as engine};

            let raw = engine.decode(*crate::vars::JWT_DEC_KEY).unwrap();
            jwt::DecodingKey::from_ed_der(&raw)
        });

        let validation = eval_once!(jwt::Validation, {
            let mut v = jwt::Validation::new(jwt::Algorithm::EdDSA);

            let fields = ["iss", "sub", "aud", "exp", "iat", "jti"]
                .into_iter()
                .map(ToOwned::to_owned);

            v.required_spec_claims.extend(fields);

            v.set_audience(&["client"]);
            v.set_issuer(&[*crate::vars::SERVE_HOST]);

            v
        });

        let data = jwt::decode::<Self>(token, key, validation)?;

        if data.claims.iat > NumericDate::now() {
            anyhow::bail!("token is issued in the future?!");
        }

        Ok(data.claims)
    }
}

impl Token {
    pub fn issue_refresh() -> Self {
        Self::Refresh {
            inner: Claims::new(core::time::Duration::from_secs(60 * 60 * 24 * 4)),
        }
    }

    pub fn issue_session() -> Self {
        Self::Session {
            inner: Claims::new(core::time::Duration::from_secs(60 * 60)),
        }
    }
}

fn available_time(duration: core::time::Duration) -> (NumericDate, NumericDate) {
    let now = NumericDate::now();
    let exp = now.after_secs(duration.as_secs());

    (now, exp)
}

impl core::ops::Deref for Token {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Refresh { inner } | Self::Session { inner } => inner,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericDate(u64);

impl NumericDate {
    pub fn now() -> Self {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("now is before epoch?!")
            .as_secs();

        Self(secs)
    }

    pub fn after_secs(&self, secs: u64) -> Self { Self(self.0 + secs) }
}

impl Serialize for NumericDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        u64::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for NumericDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u64::deserialize(deserializer).map(Self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    aud: String,
    exp: NumericDate,
    iat: NumericDate,
    jti: String,
}

impl Claims {
    fn new(duration: core::time::Duration) -> Self {
        use webauthn_rs::prelude::Uuid;

        let (from, to) = available_time(duration);
        let uuid = Uuid::new_v4().to_string();

        Self {
            iss: crate::vars::SERVE_HOST.to_owned(),
            aud: "client".to_owned(),
            exp: to,
            iat: from,
            jti: uuid,
        }
    }
}
