use core::future::Future;

use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};

use super::SessionId;
use crate::auth::Token;

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
                .map(SessionId::from);

            Self {
                refresh,
                session,
                status,
            }
        };

        async { result }
    }
}

impl Cookies {
    pub fn as_cookies(&self) -> anyhow::Result<impl Iterator<Item = Cookie>> {
        use actix_web::cookie::SameSite;

        let refresh = if let Some(ref token) = self.refresh {
            Cookie::build("refresh", token.encode()?)
                .domain(*crate::vars::SERVE_HOST)
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
                .domain(*crate::vars::SERVE_HOST)
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
            Cookie::build("status", status.to_string())
                .domain(*crate::vars::SERVE_HOST)
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

pub trait Apply {
    fn apply_cookie(&mut self, cookie: Cookie) -> &mut Self;

    fn apply_cookies(&mut self, ck: Cookies) -> anyhow::Result<&mut Self> {
        Ok(ck.as_cookies()?.fold(self, |s, c| s.apply_cookie(c)))
    }
}

impl Apply for actix_web::HttpResponseBuilder {
    fn apply_cookie(&mut self, c: Cookie) -> &mut Self { self.cookie(c) }
}
