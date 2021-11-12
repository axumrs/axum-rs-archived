use std::sync::Arc;

use axum::http::HeaderMap;

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
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<Option<AdminSession>> {
    let sess_cfg = state.clone().sess_cfg.clone();
    let cookie = get_cookie(headers, &sess_cfg.id_name);
    if let Some(session_id) = cookie {
        if !session_id.is_empty() {
            let client = state.rdc.clone();
            let redis_key = gen_redis_key(&sess_cfg, &session_id);
            let admin_session = rdb::get(client, &redis_key).await.map_err(|err| {
                tracing::error!("get session failed: {:?}", err);
                AppError::auth_error("UNAUTHENTICATED")
            })?;
            let admin_session: AdminSession =
                serde_json::from_str(&admin_session).map_err(|err| {
                    tracing::error!("des admin_session failed: {:?}", err);
                    AppError::auth_error("UNAUTHENTICATED")
                })?;
            tracing::debug!("admin session: {:?}", admin_session);
            return Ok(Some(admin_session));
        }
    }
    Ok(None)
}
