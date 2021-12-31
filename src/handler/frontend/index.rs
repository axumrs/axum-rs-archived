use axum::{http::HeaderMap, response::Html};
use reqwest::StatusCode;

use crate::{
    handler::{helper::render, redirect::redirect},
    html::frontend::index::IndexTemplate,
    Result,
};
pub async fn index() -> Result<Html<String>> {
    let handler_name = "frontend_index";
    let tmpl = IndexTemplate {};
    render(tmpl, handler_name)
}
pub async fn video() -> Result<(StatusCode, HeaderMap, ())> {
    redirect("https://www.youtube.com/channel/UCxYIyGnTIXK3oXgqZhZPpYQ")
}
