use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Extension, Form},
    http::{HeaderMap, Request, StatusCode},
};

use crate::{error::AppError, form, model::AppState, Result};

use super::redirect::redirect_with_cookie;

pub async fn admin_login(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, HeaderMap, ())> {
    redirect_with_cookie("/admin", Some("user=foo"))
}
