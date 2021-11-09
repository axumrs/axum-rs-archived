use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use std::sync::Arc;

use crate::{
    db::{tag, topic},
    handler::helper::{get_client, log_error, render},
    html::frontend::tag::{IndexTemplate, TopicsTemplate},
    model::AppState,
    Result,
};

use super::PaginationArgs;

pub async fn index(Extension(state): Extension<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "frontend_tag_index";
    let client = get_client(state, handler_name).await?;
    let tags = tag::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate { tags };
    render(tmpl, handler_name)
}

pub async fn topics(
    Extension(state): Extension<Arc<AppState>>,
    Path(name): Path<String>,
    page: Option<Query<PaginationArgs>>,
) -> Result<Html<String>> {
    let page = match page {
        Some(arg) => arg.page,
        None => 0,
    };
    let name = urlencoding::decode(&name).unwrap().into_owned();
    tracing::debug!("name: {:?}, page: {:?}", name, page);
    let handler_name = "frontend_tag_topics";
    let client = get_client(state, handler_name).await?;
    let tag = tag::find(&client, Some("name=$1 AND is_del=false"), &[&name])
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let list = topic::select_with_summary(&client, Some("$1 = ANY(tag_names)"), &[&name], page)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = TopicsTemplate {
        page,
        list,
        name,
        tag,
    };
    render(tmpl, handler_name)
}
