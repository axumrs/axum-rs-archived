use axum::{
    extract::{Extension, Form, Path, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};

use crate::{
    arg,
    db::subject,
    form,
    handler::{
        helper::{get_client, log_error, render},
        redirect::redirect,
    },
    html::backend::subject::{AddTemplate, EditTemplate, IndexTemplate},
    model::AppState,
    Result,
};
use std::sync::Arc;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::SubjectBackendQueryArg>>,
) -> Result<Html<String>> {
    let handler_name = "backend_subject_index";
    let client = get_client(&state, handler_name).await?;
    let args = args.unwrap().0;
    let q_keyword = format!("%{}%", args.keyword());
    let subject_list = subject::select(
        &client,
        "is_del=$1 AND name LIKE $2",
        &[&args.is_del(), &q_keyword],
        args.page.unwrap_or(0),
    )
    .await
    .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate {
        arg: args,
        list: subject_list,
    };
    render(tmpl, handler_name)
}
pub async fn add() -> Result<Html<String>> {
    let tmpl = AddTemplate {};
    render(tmpl, "backend_subject_add")
}
pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    form: Form<form::CreateSubject>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_subject_add_action";
    let client = get_client(&state, handler_name).await?;
    subject::create(&client, &form)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/subject?msg=专题添加成功")
}
pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let handler_name = "backend_subject_edit";
    let client = get_client(&state, handler_name).await?;
    let sub = subject::find(&client, Some("id=$1"), &[&id])
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = EditTemplate { subject: sub };
    render(tmpl, handler_name)
}
pub async fn edit_action(
    Extension(state): Extension<Arc<AppState>>,
    form: Form<form::UpdateSubject>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_subject_edit_action";
    let client = get_client(&state, handler_name).await?;
    subject::update(&client, &form)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/subject?msg=专题修改成功")
}
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_subject_del";
    let client = get_client(&state, handler_name).await?;
    subject::delete(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/subject?msg=专题删除成功")
}
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_subject_restore";
    let client = get_client(&state, handler_name).await?;
    subject::restore(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/subject?msg=专题删除成功")
}
