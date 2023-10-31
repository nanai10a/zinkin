mod cookies;
mod models;

mod uses {
    // actix-web: frequently used things
    pub use actix_web::{web, HttpResponse, Responder};
    // serde: serialization / deserialization
    pub use serde::{Deserialize, Serialize};

    // crate: models
    pub use crate::models::{self, FromModel as _};
    // crate: repositories
    pub use crate::repos::{KeyRepository, PostRepository};
    // crate: stores
    pub use crate::stores::{Entry, Store};

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

    // internal: handle cookie
    pub use super::cookies::{Apply as _, Cookies};
    // internal: models
    pub use super::models::Post;
}

#[allow(clippy::wildcard_imports)]
mod posts;

#[allow(clippy::wildcard_imports)]
mod auth;

use webauthn_rs::prelude as wan;

pub fn services<
    PR: 'static + crate::repos::PostRepository,
    KR: 'static + crate::repos::KeyRepository,
    RS: 'static + crate::stores::Store<wan::PasskeyRegistration, Key = SessionId>,
    AS: 'static + crate::stores::Store<wan::PasskeyAuthentication, Key = SessionId>,
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
        web::resource("/auth/refresh").route(web::get().to(auth::refresh)),
        web::resource("/auth/check").route(web::get().to(auth::check)),
    ];

    services![posts, auth]
}

pub use auth::SessionId;
