use std::sync::Arc;

use axum::{
    extract::{Extension, Form},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use serde_json::json;

use crate::{
    error::AppError,
    form,
    html::auth::LoginTemplate,
    model::{AdminSession, AppState},
    rdb, session,
    time::now,
    Result,
};

use super::{helper::render, redirect::redirect_with_cookie};

pub async fn admin_login_ui() -> Result<Html<String>> {
    let handler_name = "admin_login_ui";
    let tmpl = LoginTemplate {};
    render(tmpl, handler_name)
}
pub async fn admin_login(
    Extension(state): Extension<Arc<AppState>>,
    Form(login): Form<form::AdminLogin>,
) -> Result<(StatusCode, HeaderMap, ())> {
    if &login.username != "foo" || &login.password != "bar" {
        return Err(AppError::auth_error("用户名或密码错误"));
    }
    let data = json!(AdminSession {
        username: login.username,
        dateline: now(),
    });
    let data = data.to_string();
    tracing::debug!("data: {:?}", data);
    let cfg = state.sess_cfg.clone();
    let session::GeneratedKey {
        id,
        cookie_key,
        redis_key,
    } = session::gen_key(&cfg);
    tracing::debug!("{}", &redis_key);
    let client = state.rdc.clone();
    rdb::set(client, &redis_key, &data, cfg.expired)
        .await
        .map_err(AppError::from)?;
    let cookie = format!("{}={}", cookie_key, id);
    redirect_with_cookie("/admin", Some(&cookie))
}

pub async fn admin_logout() -> Result<(StatusCode, HeaderMap, ())> {
    redirect_with_cookie("/admin/subject", Some("user="))
}
