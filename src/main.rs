//#![recursion_limit = "256"]
use std::sync::Arc;

use axum::{
    extract::extractor_middleware,
    http::StatusCode,
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use axum_rs::{
    config,
    handler::{auth, backend, frontend},
    middleware::admin_auth::Auth,
    model::AppState,
};
use dotenv::dotenv;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(None, tokio_postgres::NoTls).unwrap();
    let rdc = redis::Client::open(cfg.redis.dsn).unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let state = Arc::new(AppState {
        pool,
        rdc,
        sess_cfg: cfg.session,
        hcap_cfg: cfg.hcaptcha,
    });

    let backend_router = backend::routers().layer(extractor_middleware::<Auth>());
    let frontend_router = frontend::routers();
    let static_serve = get_service(ServeDir::new("static")).handle_error(|err| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("载入静态资源出错：{}", err),
        )
    });

    let app = Router::new()
        .nest("/", frontend_router)
        .nest("/static", static_serve)
        .nest("/admin", backend_router)
        .route("/login", get(auth::admin_login_ui).post(auth::admin_login))
        .route("/logout", get(auth::admin_logout))
        .layer(AddExtensionLayer::new(state));
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
