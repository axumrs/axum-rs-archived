pub mod about;
pub mod index;
pub mod subject;
pub mod tag;
pub mod topic;

use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationArgs {
    pub page: u32,
}

pub fn routers() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/subject", get(subject::index))
        .route("/subject/:slug", get(subject::topics))
        .route("/tag", get(tag::index))
        .route("/tag/:name", get(tag::topics))
        .route("/topic", get(topic::index))
        .route("/topic/:subject_slug/:slug", get(topic::detail))
        .route(
            "/topic/get_procted_content",
            post(topic::get_procted_content),
        )
        .route("/about", get(about::index))
        .route("/video", get(index::video))
}
