use askama::Template;
use axum::{
    extract::{Extension, Form, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use deadpool_postgres::Client;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
    db::subject,
    error::AppError,
    form,
    handler::{flash, redirect::redirect},
    html::backend::subject::{AddTemplate, IndexTemplate},
    model::AppState,
    Result,
};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct QueryArg {
    pub page: Option<i32>,
    pub keyword: Option<String>,
    pub msg: Option<String>,
}

pub async fn index(args: Option<Query<QueryArg>>) -> Result<Html<String>> {
    let args = args.unwrap().0;
    //let keyword = args.keyword.unwrap_or("".to_string());
    //let page = args.page.unwrap_or(0);
    //let msg = args.msg.unwrap_or("".to_string());
    //tracing::debug!("page: {:?}, keyword: {:?}, msg: {:?}", page, keyword, msg);
    tracing::debug!("{:?}", args);
    let tmpl = IndexTemplate { msg: args.msg };
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
