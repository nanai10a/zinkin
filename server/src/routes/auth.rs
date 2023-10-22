use crate::routes::uses::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(u128);

impl SessionId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self { Self(rand::random()) }
}

use header::*;
mod header {
    use core::fmt::Display;
    use core::ops::Deref;
    use core::str::FromStr;

    use actix_web::error::ParseError;
    use actix_web::http::header as hh;
    use actix_web::http::header::{Header, TryIntoHeaderValue};
    use actix_web::HttpMessage;

    use super::token::Token;
    use super::SessionId;

    pub enum XAuthProgress {
        Unspecified,
        Challenging(SessionId),
    }

    impl FromStr for XAuthProgress {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.is_empty() {
                return Ok(Self::Unspecified);
            }

            match s.to_lowercase().split_once(' ') {
                Some(("challenging", id)) => match u128::from_str_radix(id, 16) {
                    Ok(id) => Ok(Self::Challenging(SessionId(id))),
                    Err(e) => Err(anyhow::anyhow!("parse error: {e}")),
                },
                Some((..)) | None => Err(anyhow::anyhow!("invalid progress specifier")),
            }
        }
    }

    impl Display for XAuthProgress {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Unspecified => write!(f, ""),
                Self::Challenging(id) => write!(f, "challenging {:32x}", id.0),
            }
        }
    }

    impl TryIntoHeaderValue for XAuthProgress {
        type Error = hh::InvalidHeaderValue;

        fn try_into_value(self) -> Result<hh::HeaderValue, Self::Error> {
            hh::HeaderValue::from_maybe_shared(self.to_string())
        }
    }

    impl Header for XAuthProgress {
        fn name() -> hh::HeaderName { hh::HeaderName::from_static("x-auth-progress") }

        fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
            hh::from_one_raw_str(msg.headers().get(Self::name()))
        }
    }

    pub struct Authorization(pub Option<Token>);

    impl Deref for Authorization {
        type Target = Option<Token>;

        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl FromStr for Authorization {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.is_empty() {
                return Ok(Self(None));
            }

            let Some(cred) = s.strip_prefix("Bearer ") else {
                anyhow::bail!("invalid authorization");
            };

            Ok(Self(Some(Token::decode(cred)?)))
        }
    }

    impl TryIntoHeaderValue for Authorization {
        type Error = hh::InvalidHeaderValue;

        fn try_into_value(self) -> Result<hh::HeaderValue, Self::Error> {
            let Some(token) = self.0 else {
                return Ok(hh::HeaderValue::from_static(""));
            };

            // FIXME: error handling
            let value = format!("Bearer {}", token.encode().unwrap());

            hh::HeaderValue::from_maybe_shared(value)
        }
    }

    impl Header for Authorization {
        fn name() -> hh::HeaderName { hh::HeaderName::from_static("authorization") }

        fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
            hh::from_one_raw_str(msg.headers().get(Self::name()))
        }
    }
}

mod token {
    use serde::{Deserialize, Serialize};

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
    }

    mod key {
        use std::sync::LazyLock;

        use base64::prelude::{Engine, BASE64_STANDARD as engine};
        use jsonwebtoken as jwt;

        pub static ENCODE: LazyLock<jwt::EncodingKey> = LazyLock::new(|| {
            let raw = engine.decode(*crate::envs::JWT_KEY).unwrap();
            jwt::EncodingKey::from_ed_der(&raw)
        });

        pub static DECODE: LazyLock<jwt::DecodingKey> = LazyLock::new(|| {
            let raw = engine.decode(*crate::envs::JWT_KEY).unwrap();
            jwt::DecodingKey::from_ed_der(&raw)
        });
    }

    use std::sync::LazyLock;
    static HOST_NAME: LazyLock<String> = LazyLock::new(|| {
        crate::envs::HOST_URL
            .parse::<url::Url>()
            .unwrap()
            .host_str()
            .unwrap()
            .to_owned()
    });

    impl Token {
        pub fn encode(&self) -> anyhow::Result<String> {
            use jsonwebtoken as jwt;

            let header = jwt::Header::new(jwt::Algorithm::EdDSA);

            Ok(jwt::encode(&header, self, &key::ENCODE)?)
        }

        pub fn decode(token: &str) -> anyhow::Result<Self> {
            use jsonwebtoken as jwt;

            static VALIDATION: LazyLock<jwt::Validation> = LazyLock::new(|| {
                let mut v = jwt::Validation::new(jwt::Algorithm::EdDSA);

                let fields = ["iss", "sub", "aud", "exp", "iat", "jti"]
                    .into_iter()
                    .map(ToOwned::to_owned);

                v.required_spec_claims.extend(fields);

                v.set_audience(&["client"]);
                v.set_issuer(&[&*HOST_NAME]);

                v
            });

            let data = jwt::decode::<Self>(token, &key::DECODE, &VALIDATION)?;

            if data.claims.iat < NumericDate::now() {
                anyhow::bail!("token is issued in the future?!");
            }

            Ok(data.claims)
        }
    }

    impl Token {
        pub fn issue_refresh() -> Self {
            Self::Refresh {
                inner: Claims::new(),
            }
        }

        pub fn issue_session() -> Self {
            Self::Session {
                inner: Claims::new(),
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
        fn new() -> Self {
            let (from, to) = available_time(core::time::Duration::from_secs(60 * 60 * 24 * 3));
            let uuid = webauthn_rs::prelude::Uuid::new_v4().to_string();

            Self {
                iss: HOST_NAME.clone(),
                aud: "client".to_owned(),
                exp: to,
                iat: from,
                jti: uuid,
            }
        }
    }
}

pub async fn register<KR: KeyRepository, RS: Store<wan::PasskeyRegistration, Key = SessionId>>(
    repo: web::Data<KR>,
    store: web::Data<RS>,
    site: web::Data<wan::Webauthn>,
    data: web::Json<Option<wan::RegisterPublicKeyCredential>>,
    prgs: web::Header<XAuthProgress>,
) -> impl Responder {
    let result: anyhow::Result<_> = try {
        match (&*prgs, &*data) {
            (XAuthProgress::Unspecified, None) => {
                let excludes = repo
                    .all()
                    .await?
                    .into_iter()
                    .map(|p| p.cred_id().clone())
                    .collect::<Vec<_>>();

                let (ccr, pr) = site.start_passkey_registration(
                    wan::Uuid::nil(),
                    "owner",
                    "Owner",
                    Some(excludes),
                )?;

                let id = SessionId::new();

                assert!(store.entry(id).await?.set(pr).await?);

                HttpResponse::Accepted()
                    .insert_header(XAuthProgress::Challenging(id))
                    .json(ccr)
            },

            (XAuthProgress::Challenging(id), Some(data)) => {
                let pr = store
                    .entry(*id)
                    .await?
                    .get()
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("registration isn't found"))?;

                let result = site.finish_passkey_registration(data, &pr);

                if let Ok(ref p) = result {
                    repo.push(rand::random(), p.clone()).await?;
                }

                HttpResponse::Ok().json(result.is_ok())
            },

            _ => HttpResponse::BadRequest().body(""),
        }
    };

    result_as_response!(result)
}

// FIXME: we need `break` in try block
macro try_loop($block:block) {
    try {
        loop {
            break $block;
        }
    }
}

pub async fn claim<KR: KeyRepository, AS: Store<wan::PasskeyAuthentication, Key = SessionId>>(
    repo: web::Data<KR>,
    store: web::Data<AS>,
    site: web::Data<wan::Webauthn>,
    data: web::Json<Option<wan::PublicKeyCredential>>,
    prgs: web::Header<XAuthProgress>,
    auth: web::Header<Authorization>,
) -> impl Responder {
    let result: anyhow::Result<_> = try_loop! {{
        if let Some(token) = &**auth {
            break match token {
                token::Token::Session { .. } => HttpResponse::Ok().body(""),
                token::Token::Refresh { .. } =>
                    HttpResponse::Ok().body(token::Token::issue_refresh().encode()?),
            };
        }

        match (&*prgs, &*data) {
            (XAuthProgress::Unspecified, None) => {
                let keys = repo.all().await?;

                let (rcr, pa) = site.start_passkey_authentication(&keys)?;

                let id = SessionId::new();

                assert!(store.entry(id).await?.set(pa).await?);

                HttpResponse::Continue()
                    .insert_header(XAuthProgress::Challenging(id))
                    .json(rcr)
            },

            (XAuthProgress::Challenging(id), Some(data)) => {
                let pa = match store.entry(*id).await?.get().await? {
                    None => break HttpResponse::BadRequest().body(""),
                    Some(pa) => pa,
                };

                let result = match site.finish_passkey_authentication(data, &pa) {
                    Err(_) => break HttpResponse::Ok().body(""),
                    Ok(result) => result,
                };

                dbg!(result);

                let token = token::Token::issue_refresh().encode()?;

                HttpResponse::Ok().body(token)
            },

            _ => HttpResponse::BadRequest().body(""),
        }
    }};

    result_as_response!(result)
}

pub async fn refresh(token: web::Header<Authorization>) -> impl Responder {
    let result: anyhow::Result<_> = try {
        match &**token {
            None => HttpResponse::Forbidden().body(""),

            Some(token::Token::Session { .. }) => HttpResponse::Ok().body(""),

            Some(token::Token::Refresh { .. }) =>
                HttpResponse::Ok().body(token::Token::issue_session().encode()?),
        }
    };

    result_as_response!(result)
}
