use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http,
};

use crate::error::AppError;

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
        if let Some(cookie) = cookie {
            let cookie = cookie.as_str();
            let cs: Vec<&str> = cookie.split('&').collect();
            for item in cs {
                let item: Vec<&str> = item.split('=').collect();
                let key = item[0];
                let val = item[1];
                // TODO: 加入redis
                if key == "user" && !val.is_empty() {
                    return Ok(Self {});
                }
            }
        }
        Err(AppError::auth_error("UNAUTHENTICATED"))
    }
}
