use axum::response::Html;

use crate::{handler::helper::render, html::frontend::about::IndexTemplate, Result};

pub async fn index() -> Result<Html<String>> {
    let handler_name = "frontend_about";
    let tmpl = IndexTemplate {};
    render(tmpl, handler_name)
}
