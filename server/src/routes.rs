#![allow(clippy::future_not_send)]

mod uses {
    // actix-web: macros (http methods)
    pub use actix_web::{delete, get, patch, post, put};
    // actix-web: frequently used things
    pub use actix_web::{web, Responder};
    // chrono: datetime types
    pub use chrono::{DateTime, FixedOffset};
    // serde: serialization / deserialization
    pub use serde::{Deserialize, Serialize};
    // sqlx: database
    pub use sqlx::{Executor, FromRow, Row};

    // crate: repositories
    pub use crate::repos::PostRepository;
    // crate: utilities
    pub use crate::utils::*;
    // crate: models
    pub use crate::{jsons, models};

    pub macro result_as_response($r:expr) {
        match $r {
            Ok(v) => web::Either::Right(v),
            Err(e) => {
                dbg!(e);
                web::Either::Left(actix_web::HttpResponse::InternalServerError())
            },
        }
    }
}

#[allow(clippy::wildcard_imports)]
mod posts {
    use crate::routes::uses::*;

    pub async fn get<PR: PostRepository>(repo: web::Data<PR>) -> impl Responder {
        let result: anyhow::Result<_> = try {
            let models = repo.all().await?;
            let jsons = models
                .into_iter()
                .map(jsons::Post::from_model)
                .try_collect::<Vec<_>>()?;

            web::Json(jsons)
        };

        result_as_response!(result)
    }

    #[derive(Deserialize)]
    pub struct Create {
        pub content: String,
    }

    pub async fn create<PR: PostRepository>(
        repo: web::Data<PR>,
        data: web::Json<Create>,
    ) -> impl Responder {
        let Create { content } = data.into_inner();

        let id = rand::random();
        let now = chrono::Local::now().fixed_offset();

        let model = models::Post::new(id, content, now);
        let id = model.id;

        let result: anyhow::Result<_> = try {
            repo.create(model).await?;
            let model = repo.find_one(id).await?;

            web::Json(model.map(jsons::Post::from_model).transpose()?)
        };

        result_as_response!(result)
    }

    pub mod _id_ {
        use crate::routes::uses::*;

        pub async fn get<PR: PostRepository>(
            repo: web::Data<PR>,
            id: web::Path<u32>,
        ) -> impl Responder {
            let result: anyhow::Result<_> = try {
                let model = repo.find_one(*id).await?;

                web::Json(model.map(jsons::Post::from_model).transpose()?)
            };

            result_as_response!(result)
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum Update {
            Modify { content: String },
            Deleting { is_deleted: bool },
        }

        pub async fn update<PR: PostRepository>(
            repo: web::Data<PR>,
            id: web::Path<u32>,
            data: web::Json<Update>,
        ) -> impl Responder {
            let result: anyhow::Result<_> = try {
                match data.into_inner() {
                    Update::Modify { content } => {
                        let now = chrono::Local::now().fixed_offset();
                        repo.update(*id, content, now).await?;
                    },
                    Update::Deleting { is_deleted: true } => {
                        repo.delete(*id).await?;
                    },
                    Update::Deleting { is_deleted: false } => {
                        repo.restore(*id).await?;
                    },
                }

                let model = repo.find_one(*id).await?;
                web::Json(model.map(jsons::Post::from_model).transpose()?)
            };

            result_as_response!(result)
        }
    }
}

use actix_web::dev::HttpServiceFactory;

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
