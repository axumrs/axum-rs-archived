use axum::response::Html;

use crate::{handler::helper::render, html::backend::topic::AddTemplate, Result};

pub async fn add() -> Result<Html<String>> {
    let handler_name = "backend_topic_add";
    let tmpl = AddTemplate {};
    render(tmpl, handler_name)
}
