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
