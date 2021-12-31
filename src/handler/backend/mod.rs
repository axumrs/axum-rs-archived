use axum::{http::HeaderMap, routing::get, Router};

use crate::{
    error::AppError,
    model::{AdminSession, AppState},
    rdb,
    session::gen_redis_key,
    Result,
};

use super::helper::get_cookie;

pub mod admin;
pub mod index;
pub mod subject;
pub mod tag;
pub mod topic;

pub async fn get_logined_admin(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<Option<AdminSession>> {
    let sess_cfg = state.clone().sess_cfg.clone();
    let cookie = get_cookie(headers, &sess_cfg.id_name);
    if let Some(session_id) = cookie {
        if !session_id.is_empty() {
            let redis_key = gen_redis_key(&sess_cfg, &session_id);
            let admin_session = rdb::get(&state.rdc, &redis_key).await.map_err(|err| {
                tracing::error!("get session failed: {:?}", err);
                AppError::auth_error("UNAUTHENTICATED")
            })?;
            if let Some(admin_session) = admin_session {
                let admin_session: AdminSession =
                    serde_json::from_str(&admin_session).map_err(|err| {
                        tracing::error!("des admin_session failed: {:?}", err);
                        AppError::auth_error("UNAUTHENTICATED")
                    })?;
                tracing::debug!("admin session: {:?}", admin_session);
                return Ok(Some(admin_session));
            }
        }
    }
    Ok(None)
}

pub fn routers() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/subject", get(subject::index))
        .route("/subject/add", get(subject::add).post(subject::add_action))
        .route(
            "/subject/edit/:id",
            get(subject::edit).post(subject::edit_action),
        )
        .route("/subject/del/:id", get(subject::del))
        .route("/subject/restore/:id", get(subject::restore))
        .route("/tag", get(tag::index))
        .route("/tag/add", get(tag::add).post(tag::add_action))
        .route("/tag/edit/:id", get(tag::edit).post(tag::edit_action))
        .route("/tag/del/:id", get(tag::del))
        .route("/tag/restore/:id", get(tag::restore))
        .route("/topic", get(topic::index))
        .route("/topic/add", get(topic::add).post(topic::add_action))
        .route("/topic/del/:id", get(topic::del))
        .route("/topic/restore/:id", get(topic::restore))
        .route("/topic/edit/:id", get(topic::edit).post(topic::edit_action))
        .route("/admin", get(admin::index))
        .route("/admin/add", get(admin::add).post(admin::add_action))
        .route("/admin/edit/:id", get(admin::edit).post(admin::edit_action))
        .route("/admin/del/:id", get(admin::del))
        .route("/admin/restore/:id", get(admin::restore))
}
