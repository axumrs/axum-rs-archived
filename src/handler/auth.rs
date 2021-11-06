use std::sync::Arc;

use axum::{
    extract::{Extension, Form},
    http::{HeaderMap, StatusCode},
    response::Html,
};

use crate::{error::AppError, form, html::auth::LoginTemplate, model::AppState, Result};

use super::{helper::render, redirect::redirect_with_cookie};

pub async fn admin_login_ui() -> Result<Html<String>> {
    let handler_name = "admin_login_ui";
    let tmpl = LoginTemplate {};
    render(tmpl, handler_name)
}
pub async fn admin_login(
    Extension(_state): Extension<Arc<AppState>>,
    Form(login): Form<form::AdminLogin>,
) -> Result<(StatusCode, HeaderMap, ())> {
    if &login.username != "foo" || &login.password != "bar" {
        return Err(AppError::auth_error("用户名或密码错误"));
    }
    // TODO: 加入redis
    let cookie = format!("user={}", &login.username);
    redirect_with_cookie("/admin", Some(&cookie))
}

pub async fn admin_logout() -> Result<(StatusCode, HeaderMap, ())> {
    redirect_with_cookie("/admin/subject", Some("user="))
}
