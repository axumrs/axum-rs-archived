use askama::Template;
use axum::{
    extract::{Extension, Form, Path, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use deadpool_postgres::Client;

use crate::{
    arg,
    db::subject,
    error::AppError,
    form,
    handler::{helper::get_client, redirect::redirect},
    html::backend::subject::{AddTemplate, EditTemplate, IndexTemplate},
    model::AppState,
    Result,
};
use std::sync::Arc;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::SubjectBackendQueryArg>>,
) -> Result<Html<String>> {
    let client: Client = get_client(state, "backend_subject_index").await?;
    let args = args.unwrap().0;
    let subject_list =
        subject::select(&client, "is_del=false", &[], args.page.unwrap_or(0)).await?;
    let tmpl = IndexTemplate {
        arg: args,
        subject_list,
    };
    let out = tmpl.render().map_err(AppError::tmpl_error)?;
    Ok(Html(out))
}
pub async fn add() -> Result<Html<String>> {
    let tmpl = AddTemplate {};
    let out = tmpl.render().map_err(AppError::tmpl_error)?;
    Ok(Html(out))
}
pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    form: Form<form::CreateSubject>,
) -> Result<(StatusCode, HeaderMap, ())> {
    tracing::debug!("{:?}", form);
    let client: Client = state.pool.get().await.map_err(AppError::from)?;
    subject::create(&client, &form).await?;
    redirect("/admin/subject?msg=专题添加成功")
}
pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let client: Client = state.pool.get().await.map_err(AppError::from)?;
    let sub = subject::find(&client, Some("id=$1"), &[&id]).await?;
    let tmpl = EditTemplate { subject: sub };
    let out = tmpl.render().map_err(AppError::tmpl_error)?;
    Ok(Html(out))
}
pub async fn edit_action(
    Extension(state): Extension<Arc<AppState>>,
    form: Form<form::UpdateSubject>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let client: Client = state.pool.get().await.map_err(AppError::from)?;
    subject::update(&client, &form).await?;
    redirect("/admin/subject?msg=专题修改成功")
}
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let client: Client = state.pool.get().await.map_err(AppError::from)?;
    subject::delete(&client, id).await?;
    redirect("/admin/subject?msg=专题删除成功")
}
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let client: Client = state.pool.get().await.map_err(AppError::from)?;
    subject::restore(&client, id).await?;
    redirect("/admin/subject?msg=专题恢复成功")
}
