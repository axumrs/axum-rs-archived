use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};

use super::PaginationArgs;
use crate::{
    db::{subject, topic},
    handler::helper::{get_client, log_error, render},
    html::frontend::subject::{IndexTemplate, TopicsTemplate},
    model::AppState,
    Result,
};

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    page: Option<Query<PaginationArgs>>,
) -> Result<Html<String>> {
    let page = match page {
        Some(arg) => arg.page,
        None => 0,
    };
    tracing::debug!("page: {:?}", page);
    let handler_name = "frontend_subject_index";
    let client = get_client(state, handler_name).await?;
    let list = subject::select_with_summary(&client, Some("is_del=false"), &[], page)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate { page, list };
    render(tmpl, handler_name)
}

pub async fn topics(
    Extension(state): Extension<Arc<AppState>>,
    Path(slug): Path<String>,
    page: Option<Query<PaginationArgs>>,
) -> Result<Html<String>> {
    let page = match page {
        Some(arg) => arg.page,
        None => 0,
    };
    tracing::debug!("slug: {:?}, page: {:?}", slug, page);
    let handler_name = "frontend_subject_topics";
    let client = get_client(state, handler_name).await?;
    let subj = subject::find(&client, Some("slug=$1 AND is_del=false"), &[&slug])
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let list = topic::select_with_summary(
        &client,
        Some("subject_slug=$1"),
        &[&slug],
        Some("id ASC"),
        page,
    )
    .await
    .map_err(log_error(handler_name.to_string()))?;
    let tmpl = TopicsTemplate {
        page,
        list,
        slug,
        subject: subj,
    };
    render(tmpl, handler_name)
}
