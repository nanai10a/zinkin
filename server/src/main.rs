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
#![feature(try_blocks)]
#![feature(result_flattening)]
#![feature(decl_macro)]
#![feature(fs_try_exists)]
#![feature(async_closure)]
#![feature(never_type)]
#![feature(lazy_cell)]
#![feature(impl_trait_in_assoc_type)]

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

/// define about authn / authz
pub mod auth;

pub mod vars {
    macro static_lazy($name:ident, $expr:expr) {
        pub static $name: std::sync::LazyLock<&str> = std::sync::LazyLock::new(|| $expr);
    }

    macro load_env($name:ident) {
        static_lazy!($name, std::env::var(stringify!($name)).unwrap().leak());
    }

    load_env!(LISTEN_ADDR);

    load_env!(SERVE_URL);
    static_lazy!(SERVE_HOST, {
        SERVE_URL
            .parse::<url::Url>()
            .unwrap()
            .host_str()
            .unwrap()
            .to_owned()
            .leak()
    });

    load_env!(DB_URL);

    load_env!(JWT_ENC_KEY);
    load_env!(JWT_DEC_KEY);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().pretty().init();

    let repo = actix_web::web::Data::new(repos::SqliteRepository::new(*vars::DB_URL).await?);
    let store = actix_web::web::Data::new(stores::InMemoryStore::<routes::SessionId>::new());

    let site = actix_web::web::Data::new({
        let url = webauthn_rs::prelude::Url::parse(*vars::SERVE_URL)?;
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("hostname is none"))?;

        webauthn_rs::WebauthnBuilder::new(host, &url)?.build()?
    });

    actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(3600);

        actix_web::App::new()
            .app_data(repo.clone())
            .app_data(store.clone())
            .app_data(site.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(cors)
            .wrap(actix_web::middleware::NormalizePath::trim())
            .service(routes::services::<
                repos::SqliteRepository,
                repos::SqliteRepository,
                stores::InMemoryStore<_>,
                stores::InMemoryStore<_>,
            >())
    })
    .bind(*vars::LISTEN_ADDR)?
    .run()
    .await?;

    Ok(())
}
