use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::{error::AppError, handler::backend::get_logined_admin, model::AppState};

pub struct Auth {}
#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let state = req.extensions().unwrap().get::<Arc<AppState>>().unwrap();
        let headers = req.headers().unwrap();
        let admin_session = get_logined_admin(state, headers).await?;
        if let Some(_) = admin_session {
            return Ok(Self {});
        }
        Err(AppError::auth_error("UNAUTHENTICATED"))
    }
}
