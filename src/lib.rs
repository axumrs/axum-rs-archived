pub mod config;
pub mod db;
pub mod error;
pub mod form;
pub mod handler;
pub mod html;
pub mod model;

/// 结果
type Result<T> = std::result::Result<T, self::error::AppError>;
