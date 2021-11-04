use std::{convert::Infallible, sync::Arc};

use axum::{handler::get, http::StatusCode, service, AddExtensionLayer, Router};
use axum_rs::{
    config,
    handler::{backend, frontend},
    model::AppState,
};
use dotenv::dotenv;
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let state = Arc::new(AppState { pool });

    let backend_router = Router::new()
        .route("/subject", get(backend::subject::index))
        .route(
            "/subject/add",
            get(backend::subject::add).post(backend::subject::add_action),
        )
        .route(
            "/subject/edit/:id",
            get(backend::subject::edit).post(backend::subject::edit_action),
        )
        .route("/subject/del/:id", get(backend::subject::del))
        .route("/subject/restore/:id", get(backend::subject::restore))
        .route("/tag", get(backend::tag::index))
        .route(
            "/tag/add",
            get(backend::tag::add).post(backend::tag::add_action),
        )
        .route(
            "/tag/edit/:id",
            get(backend::tag::edit).post(backend::tag::edit_action),
        )
        .route("/tag/del/:id", get(backend::tag::del))
        .route("/tag/restore/:id", get(backend::tag::restore))
        .route(
            "/topic/add",
            get(backend::topic::add).post(backend::topic::add_action),
        )
        .layer(CookieManagerLayer::new());
    let static_serve = service::get(ServeDir::new("static")).handle_error(|err| {
        Ok::<_, Infallible>((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("载入静态资源出错：{}", err),
        ))
    });

    let app = Router::new()
        .route("/", get(frontend::index::index))
        .nest("/static", static_serve)
        .nest("/admin", backend_router)
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
