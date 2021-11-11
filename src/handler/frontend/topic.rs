use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use serde::Deserialize;

use crate::{
    db::topic,
    handler::helper::{get_client, log_error, protected_content, render},
    html::frontend::topic::{DetailTemplate, IndexTemplate},
    model::AppState,
    Result,
};

use super::PaginationArgs;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    page: Option<Query<PaginationArgs>>,
) -> Result<Html<String>> {
    let page = match page {
        Some(arg) => arg.page,
        None => 0,
    };
    let handler_name = "frontend_topics_index";
    let client = get_client(state, handler_name).await?;
    let list = topic::select_with_summary(&client, None, &[], Some("id DESC"), page)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate { page, list };
    render(tmpl, handler_name)
}

#[derive(Deserialize)]
pub struct TopicArgs {
    pub subject_slug: String,
    pub slug: String,
}

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(arg): Path<TopicArgs>,
) -> Result<Html<String>> {
    let TopicArgs { subject_slug, slug } = arg;
    tracing::debug!("subject_slug: {:?}, slug: {:?}", subject_slug, slug);
    let handler_name = "frontend_topics_detail";
    let mut client = get_client(state, handler_name).await?;
    let mut result = topic::detail(&mut client, &subject_slug, &slug)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let p_html = protected_content(&result.html, 2);
    result.html = p_html;
    let tmpl = DetailTemplate { topic: result };
    render(tmpl, handler_name)
}
