use crate::{
    arg,
    db::tag,
    handler::helper::{get_client, log_error, render},
    html::backend::tag::IndexTemplate,
    model::AppState,
    Result,
};
use axum::{
    extract::{Extension, Query},
    response::Html,
};
use std::sync::Arc;

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    args: Option<Query<arg::TagBackendQueryArg>>,
) -> Result<Html<String>> {
    let handler_name = "backend_tag_index";
    let args = args.unwrap().0;
    let client = get_client(state, handler_name).await?;
    let tag_list = tag::select(&client, None, &[], args.page.unwrap_or(0))
        .await
        .map_err(log_error(handler_name.to_string()))?;
    let tmpl = IndexTemplate {
        arg: args,
        list: tag_list,
    };
    render(tmpl, handler_name)
}
