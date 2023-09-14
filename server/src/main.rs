#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
//
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data_factory(|| repos::SqlitePostRepository::new(std::path::Path::new("zinkin.db")))
            .service(routes::services::<repos::SqlitePostRepository>())
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await?;

    Ok(())
}
