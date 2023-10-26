mod cookies;
mod models;

mod uses {
    // actix-web: macros (http methods)
    pub use actix_web::{delete, get, patch, post, put};
    // actix-web: frequently used things
    pub use actix_web::{web, HttpResponse, Responder};
    // serde: serialization / deserialization
    pub use serde::{Deserialize, Serialize};
    // sqlx: database
    pub use sqlx::{Executor, FromRow, Row};
    // webauthn-rs: authentication
    pub use webauthn_rs::prelude as wan;

    // crate: models
    pub use crate::models;
    // crate: repositories
    pub use crate::repos::{KeyRepository, PostRepository};
    // crate: stores
    pub use crate::stores::{Entry, Store};
    // crate: utilities
    pub use crate::utils::*;

    pub macro try_into_responder($block:block) {{
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

    pub macro result_as_response($r:expr) {
        match $r {
            Ok(v) => web::Either::Right(v),
            Err(e) => {
                dbg!(e);
                web::Either::Left(actix_web::HttpResponse::InternalServerError())
            },
        }
    }

    // internal: handle cookie
    pub use super::cookies::{Apply as _, Cookies};
    // internal: models
    pub use super::models::{DateTime, Post, PostContent};
}

#[allow(clippy::wildcard_imports)]
mod posts;

#[allow(clippy::wildcard_imports)]
mod auth;

pub fn services<
    PR: 'static + crate::repos::PostRepository,
    KR: 'static + crate::repos::KeyRepository,
    RS: 'static + crate::stores::Store<uses::wan::PasskeyRegistration, Key = SessionId>,
    AS: 'static + crate::stores::Store<uses::wan::PasskeyAuthentication, Key = SessionId>,
>() -> impl actix_web::dev::HttpServiceFactory {
    use actix_web::{services, web};

    let posts = services![
        web::resource("/posts")
            .route(web::get().to(posts::get::<PR>))
            .route(web::post().to(posts::create::<PR>)),
        web::resource("/posts/{id}")
            .route(web::get().to(posts::_id_::get::<PR>))
            .route(web::patch().to(posts::_id_::update::<PR>)),
    ];

    let auth = services![
        web::resource("/auth/register").route(web::post().to(auth::register::<KR, RS>)),
        web::resource("/auth/claim").route(web::post().to(auth::claim::<KR, AS>)),
        web::resource("/auth/refresh").route(web::post().to(auth::refresh)),
    ];

    services![posts, auth]
}

pub use auth::SessionId;
