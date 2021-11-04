use axum::{
    extract::{Extension, Form, Query},
    http::HeaderMap,
    http::StatusCode,
    response::Html,
};

use crate::{
    arg,
    db::{subject, topic},
    form,
    handler::{
        helper::{get_client, log_error, render},
        redirect::redirect,
    },
    html::backend::topic::{AddTemplate, IndexTemplate},
    md,
    model::AppState,
    Result,
};
use std::sync::Arc;

pub async fn add(Extension(state): Extension<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "backend_topic_add";
    let client = get_client(state, handler_name).await?;
    let subjects = subject::all(&client)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = AddTemplate { subjects };
    render(tmpl, handler_name)
}

pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ct): Form<form::CreateTopic>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_topic_add";
    let html_text = md::to_html(&ct.md);
    let mut client = get_client(state, handler_name).await?;
    topic::create(&mut client, &ct, &html_text)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/topic?msg=文章添加成功")
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::BackendQueryArg>>,
) -> Result<Html<String>> {
    let handler_name = "backend_tag_index";
    let client = get_client(state, handler_name).await?;
    let args = args.unwrap().0;
    let q_keyword = format!("%{}%", args.keyword());
    let list = topic::select(
        &client,
        Some("subject_is_del=false AND is_del=$1 AND (title LIKE $2 OR subject_name LIKE $2)"),
        &[&args.is_del(), &q_keyword],
        args.page(),
    )
    .await
    .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate { list, arg: args };
    render(tmpl, handler_name)
}
