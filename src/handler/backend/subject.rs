use askama::Template;
use axum::response::Html;

use crate::{error::AppError, html::backend::subject::index::IndexTemplate, Result};
pub async fn index() -> Result<Html<String>> {
    let tmpl = IndexTemplate {
        name: "axum_rs".to_string(),
    };
    let out = tmpl.render().map_err(AppError::tmpl_error)?;
    Ok(Html(out))
}
