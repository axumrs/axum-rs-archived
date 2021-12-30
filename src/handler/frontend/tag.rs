use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use serde_json::{from_str, json};
use std::sync::Arc;

use crate::{
    cache,
    db::{tag, topic},
    handler::helper::{get_client, log_error, render},
    html::frontend::tag::{IndexTemplate, TopicsTemplate},
    model::{AppState, Tag},
    Result,
};

use super::PaginationArgs;

pub async fn index(Extension(state): Extension<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "frontend_tag_index";
    let cache_key = cache::gen_name(format!("{}:all", handler_name).as_str());
    let cached_content = cache::read(&state.rdc, &cache_key).await;
    let client = get_client(&state, handler_name).await?;
    let mut tags: Option<Vec<Tag>> = None;
    let mut flag = false;
    if let Some(cached_content) = cached_content {
        match from_str(&cached_content) {
            Ok(l) => {
                tags = Some(l);
                flag = true;
                tracing::debug!("命中缓存");
            }
            _ => {
                flag = false;
            }
        }
    };
    if !flag {
        let tags_db = tag::all(&client)
            .await
            .map_err(log_error(handler_name.to_string()))?;
        cache::write(&state.rdc, &cache_key, json!(tags_db).to_string().as_str()).await;
        tags = Some(tags_db);
    }
    let tmpl = IndexTemplate {
        tags: tags.unwrap(),
    };
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
    let client = get_client(&state, handler_name).await?;
    let tag = tag::find(&client, Some("name=$1 AND is_del=false"), &[&name])
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let list = topic::select_with_summary(
        &client,
        Some("$1 = ANY(tag_names)"),
        &[&name],
        Some("id ASC"),
        page,
    )
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
