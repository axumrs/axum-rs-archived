use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http,
};

use crate::{
    error::AppError,
    model::{AdminSession, AppState},
    rdb,
    session::gen_redis_key,
};

pub struct Auth {}
#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = req
            .headers()
            .unwrap()
            .get(http::header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());
        tracing::debug!(" {} cookie: {:?}", req.uri(), cookie);
        let state = req.extensions().unwrap().get::<Arc<AppState>>().unwrap();
        let client = state.rdc.clone();
        let sess_cfg = state.sess_cfg.clone();
        let id_name = sess_cfg.id_name.clone();

        if let Some(cookie) = cookie {
            let cookie = cookie.as_str();
            let cs: Vec<&str> = cookie.split(';').collect();
            for item in cs {
                let item: Vec<&str> = item.split('=').collect();
                let key = item[0];
                let val = item[1];
                let key = key.trim();
                let val = val.trim();
                if key == id_name && !val.is_empty() {
                    let redis_key = gen_redis_key(&sess_cfg, val);
                    let admin_session = rdb::get(client, &redis_key).await.map_err(|err| {
                        tracing::error!("{:?}", err);
                        AppError::from(err)
                    })?;
                    let admin_session: AdminSession = serde_json::from_str(&admin_session).unwrap();
                    tracing::debug!("admin session: {}", admin_session.username);
                    return Ok(Self {});
                }
            }
        }
        Err(AppError::auth_error("UNAUTHENTICATED"))
    }
}
