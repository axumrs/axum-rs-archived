use axum::http::{HeaderMap, StatusCode};

/// 重定向
pub fn redirect(url: &str) -> crate::Result<(StatusCode, HeaderMap, ())> {
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, header, ()))
}
