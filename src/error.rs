//! Error handling for the Battlefield 2042 stats library

use thiserror::Error;

/// Result type alias for this library
pub type Result<T> = std::result::Result<T, StatsError>;

/// Main error type for the stats library
#[derive(Error, Debug)]
pub enum StatsError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(#[from] sqlx::Error),

    #[error("Database query failed: {0}")]
    QueryFailed(String),

    #[error("JSON parsing error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
