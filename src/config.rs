//! 配置

use serde::Deserialize;

/// Web配置
#[derive(Deserialize)]
pub struct WebConfig {
    ///  web服务监听地址
    pub addr: String,
    /// 安全key
    pub secret_key: String,
}

/// 配置
#[derive(Deserialize)]
pub struct Config {
    /// web配置
    pub web:WebConfig,
    /// Postgres配置
    pub pg:deadpool_postgres::Config,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
