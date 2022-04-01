//! 自定义错误

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::html::err::ErrTemplate;

/// 应用错误类型
#[derive(Debug)]
pub enum AppErrorType {
    /// 数据库错误
    DbError,
    /// 未找到
    NotFound,
    /// 已存在
    IsExists,
    /// 模板
    Template,
    AuthError,
    RedisError,
    HttpError,
    JsonError,
    ProtectedContentError,
    /// 配置
    Config,
    /// 通用错误
    Common,
}

/// 应用错误
#[derive(Debug)]
pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}
impl AppError {
    /// 从其它错误中实例化
    pub fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }
    /// 通过文本实例化
    pub fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }
    /// 处理数据库错误
    fn db_error(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbError)
    }
    pub fn db_error_from_str(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::DbError)
    }
    /// 处理未找到
    pub fn not_found(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::NotFound)
    }
    pub fn not_found_from_err(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::NotFound)
    }
    pub fn is_exists(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::IsExists)
    }
    pub fn tmpl_error(err: impl ToString) -> Self {
        Self {
            message: Some("渲染模板出错".to_owned()),
            cause: Some(err.to_string()),
            error_type: AppErrorType::Template,
        }
    }
    pub fn protected_content(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::ProtectedContentError)
    }
    pub fn auth_error(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::AuthError)
    }
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Template => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::ProtectedContentError => StatusCode::OK,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}
impl std::error::Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_error(err)
    }
}
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_error(err)
    }
}
impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::tmpl_error(err)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        Self::from_err(err, AppErrorType::RedisError)
    }
}
impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::from_err(err, AppErrorType::Common)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::from_err(err, AppErrorType::JsonError)
    }
}
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        Self::from_err(err, AppErrorType::Config)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = (&self).status_code();
        let msg = match self {
            AppError {
                message: Some(msg), ..
            } => msg.clone(),
            AppError {
                error_type: AppErrorType::DbError,
                ..
            } => "数据库操作失败".to_string(),
            AppError {
                error_type: AppErrorType::NotFound,
                ..
            } => "没有找到".to_string(),
            AppError {
                error_type: AppErrorType::Template,
                ..
            } => "模板渲染出错".to_string(),
            _ => "发生错误".to_string(),
        };
        let tmpl = ErrTemplate {
            err: msg.to_string(),
        };
        let msg = tmpl.render().unwrap_or(msg.to_string());
        (status_code, Html(msg)).into_response()
    }
}
