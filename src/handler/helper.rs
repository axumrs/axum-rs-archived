use std::sync::Arc;

use crate::error::AppError;
use crate::model::AppState;
use crate::Result;
use deadpool_postgres::Client;

pub async fn get_client(state: Arc<AppState>, hander_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("无法获取数据库连接：{},  {}", err, hander_name);
        AppError::from(err)
    })
}

pub async fn log_error(hander_name: &'static str) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("操作失败：{},  {}", err, hander_name);
        AppError::from(err)
    })
}
