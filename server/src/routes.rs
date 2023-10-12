mod models;

mod uses {
    // actix-web: macros (http methods)
    pub use actix_web::{delete, get, patch, post, put};
    // actix-web: frequently used things
    pub use actix_web::{web, Responder};
    // serde: serialization / deserialization
    pub use serde::{Deserialize, Serialize};
    // sqlx: database
    pub use sqlx::{Executor, FromRow, Row};

    // crate: models
    pub use crate::models;
    // crate: repositories
    pub use crate::repos::PostRepository;
    // crate: utilities
    pub use crate::utils::*;

    pub macro result_as_response($r:expr) {
        match $r {
            Ok(v) => web::Either::Right(v),
            Err(e) => {
                dbg!(e);
                web::Either::Left(actix_web::HttpResponse::InternalServerError())
            },
        }
    }

    // internal: models
    pub use super::models::{DateTime, Post, PostContent};
}

#[allow(clippy::future_not_send)]
#[allow(clippy::wildcard_imports)]
mod posts;

use crate::repos::PostRepository;

pub fn services<R: 'static + PostRepository>() -> impl HttpServiceFactory {
    use actix_web::web;

    vec![
        web::resource("/posts")
            .route(web::get().to(posts::get::<R>))
            .route(web::post().to(posts::create::<R>)),
        web::resource("/posts/{id}")
            .route(web::get().to(posts::_id_::get::<R>))
            .route(web::patch().to(posts::_id_::update::<R>)),
    ]
}
