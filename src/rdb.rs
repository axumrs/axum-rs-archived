//! redis 操作

use redis::{aio::Connection, AsyncCommands, Client};

use crate::{error::AppError, Result};

async fn get_conn(client: Client) -> Result<Connection> {
    client.get_async_connection().await.map_err(AppError::from)
}

pub async fn set(client: Client, key: &str, value: &str, sec: usize) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.set_ex(key, value, sec).await.map_err(AppError::from)?;
    Ok(())
}
pub async fn get(client: Client, key: &str) -> Result<String> {
    let mut conn = get_conn(client).await?;
    //let s: String = conn.get(key).await.map_err(AppError::from)?;
    let s: String = conn.get(key).await.map_err(|err| {
        tracing::debug!("{:?}", err);
        AppError::from(err)
    })?;
    Ok(s)
}
pub async fn is_exists(client: Client, key: &str) -> Result<bool> {
    let mut conn = get_conn(client).await?;
    conn.exists(key).await.map_err(AppError::from)?;
    Ok(true)
}
pub async fn del(client: Client, key: &str) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.del(key).await.map_err(AppError::from)
}
