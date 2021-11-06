use axum::http::{HeaderMap, StatusCode};

/// 重定向
pub fn redirect(url: &str) -> crate::Result<(StatusCode, HeaderMap, ())> {
    redirect_with_cookie(url, None)
}

/// 重定向
pub fn redirect_with_cookie(
    url: &str,
    cookie: Option<&str>,
) -> crate::Result<(StatusCode, HeaderMap, ())> {
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.parse().unwrap());
    if let Some(cookie) = cookie {
        if !cookie.is_empty() {
            header.insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
        }
    }
    Ok((StatusCode::FOUND, header, ()))
}
