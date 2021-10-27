use std::sync::Arc;

use crate::error::AppError;
use crate::model::AppState;
use crate::Result;
use askama::Template;
use axum::response::Html;
use deadpool_postgres::Client;

pub async fn get_client(state: Arc<AppState>, handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("无法获取数据库连接：{:?},  {}", err, handler_name);
        AppError::from(err)
    })
}

pub fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("操作失败：{:?},  {}", err, handler_name);
        err
    })
}

pub fn render<T: Template>(tmpl: T, handler_name: &str) -> Result<Html<String>> {
    let out = tmpl.render().map_err(|err| {
        tracing::error!("模板渲染出错：{:?}, {}", err, handler_name);
        AppError::from(err)
    })?;
    Ok(Html(out))
}
