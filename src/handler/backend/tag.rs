use crate::{
    arg,
    db::tag,
    form,
    handler::{
        helper::{get_client, log_error, render},
        redirect::redirect,
    },
    html::backend::tag::{AddTemplate, EditTemplate, IndexTemplate},
    model::AppState,
    Result,
};
use axum::{
    extract::{Extension, Form, Path, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use std::sync::Arc;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::TagBackendQueryArg>>,
) -> Result<Html<String>> {
    let handler_name = "backend_tag_index";
    let args = args.unwrap().0;
    let client = get_client(state, handler_name).await?;
    let tag_list = tag::select(&client, None, &[], args.page.unwrap_or(0))
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate {
        arg: args,
        list: tag_list,
    };
    render(tmpl, handler_name)
}
pub async fn add() -> Result<Html<String>> {
    let tmpl = AddTemplate {};
    render(tmpl, "backend_tag_add")
}
pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ct): Form<form::CreateTag>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_tag_add_action";
    let client = get_client(state, handler_name).await?;
    tag::create(&client, &ct)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/tag?msg=标签添加成功")
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let handler_name = "backend_tag_edit";
    let client = get_client(state, handler_name).await?;
    let tag = tag::find(&client, Some("id=$1"), &[&id])
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = EditTemplate { tag };
    render(tmpl, handler_name)
}
pub async fn edit_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ut): Form<form::UpdateTag>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_tag_edit_action";
    let client = get_client(state, handler_name).await?;
    tag::update(&client, &ut)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/tag?msg=标签修改成功")
}
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_tag_del";
    let client = get_client(state, handler_name).await?;
    tag::del(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/tag?msg=标签删除成功")
}
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_tag_restore";
    let client = get_client(state, handler_name).await?;
    tag::restore(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/tag?msg=标签恢复成功")
}
