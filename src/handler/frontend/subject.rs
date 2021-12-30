use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use serde_json::{from_str, json};

use super::PaginationArgs;
use crate::{
    cache,
    db::{pagination::Pagination, subject, topic},
    handler::helper::{get_client, log_error, render},
    html::frontend::subject::{IndexTemplate, TopicsTemplate},
    model::{AppState, Subject},
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
    let handler_name = "frontend_subject_index";
    let cache_key = cache::gen_name(format!("{}:{}", handler_name, page).as_str());
    let cache_client = state.clone().rdc.clone();
    let cached_content = cache::read(cache_client.clone(), &cache_key).await;
    tracing::debug!("page: {:?}", page);
    let client = get_client(&state, handler_name).await?;
    let mut list: Option<Pagination<Vec<Subject>>> = None;
    let mut flag = false;
    if let Some(cached_content) = cached_content {
        match from_str(&cached_content) {
            Ok(l) => {
                list = Some(l);
                flag = true;
                tracing::debug!("命中缓存");
            }
            _ => {
                flag = false;
            }
        };
    }
    if !flag {
        let list_db = subject::select_with_summary(&client, Some("is_del=false"), &[], page)
            .await
            .map_err(log_error(handler_name.to_string()))?;
        cache::write(
            cache_client,
            &cache_key,
            json!(list_db).to_string().as_str(),
        )
        .await;
        list = Some(list_db);
    };

    let tmpl = IndexTemplate {
        page,
        list: list.unwrap(),
    };
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
    let client = get_client(&state, handler_name).await?;
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
