use std::convert::Infallible;

use axum::{
    handler::{get, Handler},
    http::StatusCode,
    service, Router,
};
use axum_rs::{
    config,
    handler::{backend, frontend},
};
use dotenv::dotenv;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let backend_router = Router::new().route("/subject", get(backend::subject::index));
    let static_serve = service::get(ServeDir::new("axum-rs/static")).handle_error(|err| {
        Ok::<_, Infallible>((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("载入静态资源出错：{}", err),
        ))
    });

    let app = Router::new()
        .route("/", get(frontend::index::index))
        .nest("/static", static_serve)
        .nest("/admin", backend_router)
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
