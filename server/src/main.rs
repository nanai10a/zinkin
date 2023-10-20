#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
//
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::significant_drop_in_scrutinee)]
#![allow(clippy::trivially_copy_pass_by_ref)]
//
// FIXME: currently ignored, but must think about there
#![allow(async_fn_in_trait)]
#![allow(clippy::future_not_send)]
//
#![feature(iterator_try_collect)]
#![feature(stmt_expr_attributes)]
#![feature(slice_as_chunks)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(try_blocks)]
#![feature(async_fn_in_trait)]
#![feature(result_flattening)]
#![feature(decl_macro)]
#![feature(fs_try_exists)]
#![feature(async_closure)]
#![feature(never_type)]

/// defines models of domain
pub mod models;

/// defines models for convert to / from database data
pub mod rows;

/// defines endpoints for actix-web
pub mod routes;

/// functional utilities
pub mod utils;

/// defines repositories of models
pub mod repos;

/// defines stores of models
pub mod stores;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let repo = actix_web::web::Data::new({
        repos::SqliteRepository::new(std::path::Path::new("zinkin.db")).await?
    });

    #[rustfmt::skip]
    let store = actix_web::web::Data::new({
        stores::InMemoryStore::<routes::SessionId>::new()
    });

    let site = actix_web::web::Data::new({
        let url = webauthn_rs::prelude::Url::parse("https://localhost:9090")?;
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("hostname is none"))?;

        webauthn_rs::WebauthnBuilder::new(host, &url)?.build()?
    });

    actix_web::HttpServer::new(move || {
        let cors = {
            actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        };

        actix_web::App::new()
            .wrap(cors)
            .app_data(repo.clone())
            .app_data(store.clone())
            .app_data(site.clone())
            .service(routes::services::<
                repos::SqliteRepository,
                repos::SqliteRepository,
                stores::InMemoryStore<_>,
                stores::InMemoryStore<_>,
            >())
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await?;

    Ok(())
}
