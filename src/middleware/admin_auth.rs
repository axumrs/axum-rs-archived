use std::{convert::Infallible, sync::Arc};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http,
};

use crate::model::AppState;
pub struct Auth {
    pub id: String,
}
#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = Infallible;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = req
            .headers()
            .unwrap()
            .get(http::header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());
        tracing::debug!(" {} cookie: {:?}", req.uri(), cookie);
        Ok(Self {
            id: "123".to_string(),
        })
    }
}
