//! Configuration-related data structures

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    /// Create database URL from components
    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    /// Create from environment variables
    pub fn from_env() -> crate::Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        Ok(Self {
            host: std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("POSTGRES_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| crate::StatsError::ConfigError("Invalid POSTGRES_PORT".to_string()))?,
            database: std::env::var("POSTGRES_DB").unwrap_or_else(|_| "bf2042_stats".to_string()),
            username: std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string()),
        })
    }
}
