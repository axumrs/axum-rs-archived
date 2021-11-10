use std::sync::Arc;

use axum::{
    extract::{Extension, Form},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use serde_json::json;

use crate::{
    error::AppError,
    form, hcaptcha,
    html::auth::LoginTemplate,
    model::{AdminSession, AppState},
    rdb,
    session::{self, gen_redis_key},
    time::now,
    Result,
};

use super::{helper::render, redirect::redirect_with_cookie};

pub async fn admin_login_ui(Extension(state): Extension<Arc<AppState>>) -> Result<Html<String>> {
    let handler_name = "admin_login_ui";
    let site_key = state.hcap_cfg.site_key.clone();
    let tmpl = LoginTemplate { site_key };
    render(tmpl, handler_name)
}
pub async fn admin_login(
    Extension(state): Extension<Arc<AppState>>,
    Form(login): Form<form::AdminLogin>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let is_valid = hcaptcha::verify(
        login.hcaptcha_response.clone(),
        state.hcap_cfg.secret_key.clone(),
    )
    .await?;
    if !is_valid {
        return Err(AppError::auth_error("人机验证失败"));
    }
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

pub async fn admin_logout(
    Extension(state): Extension<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ())> {
    let cfg = state.sess_cfg.clone();
    let cookie = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());
    if let Some(cookie) = cookie {
        let cookie = cookie.as_str();
        let cs: Vec<&str> = cookie.split(';').collect();
        for item in cs {
            let item: Vec<&str> = item.split('=').collect();
            let key = item[0];
            let val = item[1];
            let key = key.trim();
            let val = val.trim();
            if key == &cfg.id_name && !val.is_empty() {
                let client = state.rdc.clone();
                let redis_key = gen_redis_key(&cfg, val);
                rdb::del(client, &redis_key).await?;
            }
        }
    }
    let cookie_logout = format!("{}=", &cfg.id_name);
    redirect_with_cookie("/login", Some(&cookie_logout))
}
