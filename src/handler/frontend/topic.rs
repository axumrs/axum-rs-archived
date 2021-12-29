use std::sync::Arc;

use axum::{
    extract::{Extension, Form, Path, Query},
    response::Html,
    Json,
};
use serde::Deserialize;
use serde_json::from_str;

use crate::{
    db::topic,
    error::AppError,
    form,
    handler::helper::{get_client, log_error, protected_content, render, ProtectedContent},
    hcaptcha,
    html::frontend::topic::{DetailTemplate, IndexTemplate},
    model::AppState,
    rdb, Result,
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
    let client = get_client(state.clone(), handler_name).await?;
    let mut result = topic::detail(&client, &subject_slug, &slug)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let site_key = state.clone().hcap_cfg.site_key.clone();
    let (p_html, uuids) = protected_content(&result.html, state.rdc.clone(), &site_key).await;
    result.html = p_html;
    let tmpl = DetailTemplate {
        topic: result,
        uuids,
    };
    render(tmpl, handler_name)
}

pub async fn get_procted_content(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::GetProctedContent>,
) -> Result<Json<ProtectedContent>> {
    let handler_name = "frontend_topics_get_procted_content";
    let is_valid = hcaptcha::verify(
        frm.hcaptcha_response,
        state.clone().hcap_cfg.secret_key.clone(),
    )
    .await
    .map_err(log_error(handler_name.to_string()))?;
    if !is_valid {
        return Err(AppError::from_str(
            "人机验证失败",
            crate::error::AppErrorType::Common,
        ));
    };
    let client = state.rdc.clone();
    let redis_key = format!("protected_content:{}", &frm.id);
    let s = rdb::get(client, &redis_key)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    if let Some(s) = s {
        let r: ProtectedContent = from_str(&s).unwrap();
        return Ok(Json(r));
    }
    Err(AppError::not_found("没有找到指定的内容"))
}
