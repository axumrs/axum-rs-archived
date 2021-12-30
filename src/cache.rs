use redis::Client;

use crate::rdb;

const PREFIX: &str = "axum_rs:cache:";
const EXPIRED: usize = 10;

pub fn gen_name(key: &str) -> String {
    format!("{}{}", PREFIX, key)
}

pub async fn write(client: &Client, key: &str, value: &str) {
    let key = gen_name(key);
    match rdb::set(&client, &key, value, EXPIRED).await {
        Err(err) => {
            tracing::error!("{:?}", err);
        }
        _ => {}
    };
}

pub async fn read(client: &Client, key: &str) -> Option<String> {
    let key = gen_name(key);
    let value = rdb::get(&client, &key).await;
    match value {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("{:?}", err);
            None
        }
    }
}
