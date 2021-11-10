use axum::response::Html;

use crate::{handler::helper::render, html::backend::index::IndexTemplate, Result};

pub async fn index() -> Result<Html<String>> {
    let handler_name = "backend_index";
    let tmpl = IndexTemplate {};
    render(tmpl, handler_name)
}
