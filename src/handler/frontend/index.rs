use axum::response::Html;

use crate::{handler::helper::render, html::frontend::index::IndexTemplate, Result};
pub async fn index() -> Result<Html<String>> {
    let handler_name = "frontend_index";
    let tmpl = IndexTemplate {};
    render(tmpl, handler_name)
}
