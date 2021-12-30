use std::sync::Arc;

use axum::{
    extract::{Extension, Form},
    http::{HeaderMap, StatusCode},
    response::Html,
};
use serde_json::json;

use crate::{
    db::admin,
    error::AppError,
    form,
    handler::helper::{get_client, get_cookie, log_error},
    hcaptcha,
    html::auth::LoginTemplate,
    model::{AdminSession, AppState},
    password, rdb,
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
    let handler_name = "auth_login";
    let is_valid = hcaptcha::verify(
        login.hcaptcha_response.clone(),
        state.hcap_cfg.secret_key.clone(),
    )
    .await?;
    if !is_valid {
        return Err(AppError::auth_error("人机验证失败"));
    }
    let client = get_client(&state, handler_name).await?;
    let login_admin = admin::find(&client, &login.username)
        .await
        .map_err(log_error(handler_name.to_string()))?;
    tracing::debug!("{:?}", password::hash(&login.password));
    if !password::verify(&login.password, &login_admin.password)? {
        return Err(AppError::auth_error("用户名或密码错误"));
    }
    let cfg = state.sess_cfg.clone();
    let data = json!(AdminSession {
        id: login_admin.id,
        username: login_admin.username,
        dateline: now() + cfg.expired as i32,
        password: login_admin.password,
        is_sys: login_admin.is_sys,
    });
    let data = data.to_string();
    tracing::debug!("data: {:?}", data);
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
    let cookie = get_cookie(&headers, &cfg.id_name);
    if let Some(val) = cookie {
        let client = state.rdc.clone();
        let redis_key = gen_redis_key(&cfg, &val);
        rdb::del(client, &redis_key).await?;
    }
    let cookie_logout = format!("{}=", &cfg.id_name);
    redirect_with_cookie("/login", Some(&cookie_logout))
}
