use crate::routes::uses::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(u128);

impl SessionId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self { Self(rand::random()) }
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

        pub fn is_refresh(&self) -> bool { matches!(self, Self::Refresh { .. }) }

        pub fn is_session(&self) -> bool { matches!(self, Self::Session { .. }) }
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
    pub static HOST_NAME: LazyLock<String> = LazyLock::new(|| {
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
                iss: HOST_NAME.clone(),
                aud: "client".to_owned(),
                exp: to,
                iat: from,
                jti: uuid,
            }
        }
    }
}

use cookies::Cookies;
mod cookies {
    use core::future::Future;

    use actix_web::cookie::Cookie;
    use actix_web::dev::Payload;
    use actix_web::{Error, FromRequest, HttpRequest};

    use super::token::Token;
    use super::SessionId;

    pub struct Cookies {
        pub refresh: Option<Token>,
        pub session: Option<Token>,
        pub status: Option<SessionId>,
    }

    impl FromRequest for Cookies {
        type Error = Error;

        type Future = impl Future<Output = Result<Self, Self::Error>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let result: Result<Self, Self::Error> = try {
                let refresh = req
                    .cookie("refresh")
                    .and_then(|c| Token::decode(c.value()).ok())
                    .and_then(|t| t.is_refresh().then_some(t));

                let session = req
                    .cookie("session")
                    .and_then(|c| Token::decode(c.value()).ok())
                    .and_then(|t| t.is_session().then_some(t));

                let status = req
                    .cookie("status")
                    .and_then(|c| u128::from_str_radix(c.value(), 16).ok())
                    .map(SessionId);

                Self {
                    refresh,
                    session,
                    status,
                }
            };

            async { result }
        }
    }

    use actix_web::cookie::SameSite;

    impl Cookies {
        pub fn as_cookies(&self) -> anyhow::Result<impl Iterator<Item = Cookie>> {
            let refresh = if let Some(ref token) = self.refresh {
                Cookie::build("refresh", token.encode()?)
                    .domain(super::token::HOST_NAME.clone())
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .secure(true)
                    .finish()
            } else {
                let mut c = Cookie::new("refresh", "");
                c.make_removal();

                c
            };

            let session = if let Some(ref token) = self.session {
                Cookie::build("session", token.encode()?)
                    .domain(super::token::HOST_NAME.clone())
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .secure(true)
                    .finish()
            } else {
                let mut c = Cookie::new("session", "");
                c.make_removal();

                c
            };

            let status = if let Some(ref status) = self.status {
                Cookie::build("status", format!("{:32x}", status.0))
                    .domain(super::token::HOST_NAME.clone())
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .secure(true)
                    .finish()
            } else {
                let mut c = Cookie::new("status", "");
                c.make_removal();

                c
            };

            Ok([refresh, session, status].into_iter())
        }
    }
}

macro try_into_responder($block:block) {{
    use std::error::Error;

    let result: Result<HttpResponse, Box<dyn Error>> = try $block;

    let casted = match result {
        Ok(r) => return r,
        Err(boxed) => boxed.downcast::<actix_web::Error>(),
    };

    let any = match casted {
        Ok(e) => return e.error_response(),
        Err(e) => e,
    };

    tracing::error!(%any, "observed uncaught error (respond as 500)");

    HttpResponse::InternalServerError().finish()
}}

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
                    wan::Uuid::new_v4(),
                    "owner",
                    "owner",
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
