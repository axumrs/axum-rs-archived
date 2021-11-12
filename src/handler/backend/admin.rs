use crate::{
    arg,
    db::admin,
    error::AppError,
    form::{CreateAdmin, UpdateAdmin},
    handler::{
        helper::{get_client, log_error, render},
        redirect::redirect,
    },
    html::backend::admin::{AddTemplate, EditTemplate, IndexTemplate},
    model::AppState,
    password, Result,
};
use axum::{
    extract::{Extension, Form, Path, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use std::sync::Arc;

use super::get_logined_admin;

pub async fn add() -> Result<Html<String>> {
    let handler_name = "backend_admin_add";
    let tmpl = AddTemplate {};
    render(tmpl, handler_name)
}
pub async fn add_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ca): Form<CreateAdmin>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_admin_add_action";
    if ca.password.is_empty() {
        return Err(AppError::from_str(
            "请输入密码",
            crate::error::AppErrorType::Common,
        ));
    }
    if &ca.password != &ca.re_password {
        return Err(AppError::from_str(
            "两次输入的密码不一致",
            crate::error::AppErrorType::Common,
        ));
    }
    let client = get_client(state, handler_name).await?;
    let mut ca = CreateAdmin { ..ca };
    ca.password = password::hash(&ca.password)?;
    admin::create(&client, ca)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/admin?msg=账号添加成功")
}
pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::BackendQueryArg>>,
) -> Result<Html<String>> {
    let handler_name = "backend_admin_index";
    let args = args.unwrap();
    let q_keyword = format!("%{}%", args.keyword());
    let client = get_client(state, handler_name).await?;
    let admin_list = admin::select(
        &client,
        Some("is_del=$1 AND username ILIKE $2"),
        &[&args.is_del(), &q_keyword],
        args.page(),
    )
    .await
    .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate {
        list: admin_list,
        arg: args.0,
    };
    render(tmpl, handler_name)
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let handler_name = "backend_admin_edit";
    let client = get_client(state, handler_name).await?;
    let item = admin::find_by_id(&client, id)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = EditTemplate { admin: item };
    render(tmpl, handler_name)
}
pub async fn edit_action(
    Extension(state): Extension<Arc<AppState>>,
    Form(ua): Form<UpdateAdmin>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_admin_edit_action";
    if ua.new_password.is_empty() {
        return Err(AppError::from_str(
            "请输入新密码",
            crate::error::AppErrorType::Common,
        ));
    }
    if &ua.new_password != &ua.re_password {
        return Err(AppError::from_str(
            "两次输入的密码不一致",
            crate::error::AppErrorType::Common,
        ));
    }
    let admin_session = get_logined_admin(&state, &headers).await?;
    if admin_session.is_none() {
        return Err(AppError::auth_error("UNAUTHENTICATED"));
    }
    let admin_session = admin_session.unwrap();
    if !password::verify(&ua.password, &admin_session.password)? {
        return Err(AppError::auth_error("你输入的密码错误"));
    };
    let mut ua = UpdateAdmin { ..ua };
    ua.new_password = password::hash(&ua.new_password)?;
    let client = get_client(state, handler_name).await?;
    admin::update(&client, ua).await?;
    redirect("/admin/admin?msg=修改成功")
}
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_admin_del";
    let client = get_client(state, handler_name).await?;
    admin::del_or_restore(&client, id, true)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/admin?msg=删除成功")
}
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let handler_name = "backend_admin_restore";
    let client = get_client(state, handler_name).await?;
    admin::del_or_restore(&client, id, false)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    redirect("/admin/admin?msg=恢复成功")
}
