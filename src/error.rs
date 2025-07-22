//! Error handling for the Battlefield 2042 stats library

/// Result type alias for this library
pub type Result<T> = std::result::Result<T, StatsError>;

/// Main error type for the stats library
#[derive(Debug)]
pub enum StatsError {
    ConnectionFailed,
    QueryFailed(String),
    ParseError,
    IoError,
    ConfigError(String),
}
