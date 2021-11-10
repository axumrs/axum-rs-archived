use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::extractor_middleware, handler::get, http::StatusCode, service, AddExtensionLayer,
    Router,
};
use axum_rs::{
    config,
    handler::{auth, backend, frontend},
    middleware::admin_auth::Auth,
    model::AppState,
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
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();
    let rdc = redis::Client::open(cfg.redis.dsn).unwrap();
    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let state = Arc::new(AppState {
        pool,
        rdc,
        sess_cfg: cfg.session,
    });

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
        .route("/topic", get(backend::topic::index))
        .route(
            "/topic/add",
            get(backend::topic::add).post(backend::topic::add_action),
        )
        .route("/topic/del/:id", get(backend::topic::del))
        .route("/topic/restore/:id", get(backend::topic::restore))
        .route(
            "/topic/edit/:id",
            get(backend::topic::edit).post(backend::topic::edit_action),
        )
        .layer(extractor_middleware::<Auth>());
    let frontend_router = Router::new()
        .route("/", get(frontend::index::index))
        .route("/subject", get(frontend::subject::index))
        .route("/subject/:slug", get(frontend::subject::topics))
        .route("/tag", get(frontend::tag::index))
        .route("/tag/:name", get(frontend::tag::topics))
        .route("/topic", get(frontend::topic::index))
        .route("/topic/:subject_slug/:slug", get(frontend::topic::detail))
        .route("/about", get(frontend::about::index));
    let static_serve = service::get(ServeDir::new("static")).handle_error(|err| {
        Ok::<_, Infallible>((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("载入静态资源出错：{}", err),
        ))
    });

    let app = Router::new()
        .nest("/", frontend_router)
        .nest("/static", static_serve)
        .nest("/admin", backend_router)
        .route("/login", get(auth::admin_login_ui).post(auth::admin_login))
        .route("/logout", get(auth::admin_logout))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
