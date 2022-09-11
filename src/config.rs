//! 配置

use serde::Deserialize;

use crate::{error::AppError, Result};

/// Web配置
#[derive(Deserialize)]
pub struct WebConfig {
    ///  web服务监听地址
    pub addr: String,
    /// 安全key
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct RedisConfig {
    pub dsn: String,
}
#[derive(Deserialize, Clone)]
pub struct SessionConfig {
    pub prefix: String,
    pub id_name: String,
    pub expired: usize,
}
#[derive(Deserialize, Clone)]
pub struct HCaptchaConfig {
    pub site_key: String,
    pub secret_key: String,
}
#[derive(Deserialize, Clone)]
pub struct ReCaptchaConfig {
    pub site_key: String,
    pub secret_key: String,
}

/// 配置
#[derive(Deserialize)]
pub struct Config {
    /// web配置
    pub web: WebConfig,
    /// Postgres配置
    pub pg: deadpool_postgres::Config,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub hcaptcha: HCaptchaConfig,
    pub recaptcha: ReCaptchaConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(AppError::from)?
            .try_deserialize()
            .map_err(AppError::from)
    }
}
