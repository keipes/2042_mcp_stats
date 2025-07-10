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
        
        // Try DATABASE_URL first
        if let Ok(url) = std::env::var("DATABASE_URL") {
            return Self::from_url(&url);
        }

        // Fall back to individual components
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

    /// Parse from DATABASE_URL
    fn from_url(url: &str) -> crate::Result<Self> {
        // This is a simplified parser - in production you'd want a proper URL parser
        let url = url.strip_prefix("postgresql://")
            .or_else(|| url.strip_prefix("postgres://"))
            .ok_or_else(|| crate::StatsError::ConfigError("Invalid database URL scheme".to_string()))?;

        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() != 2 {
            return Err(crate::StatsError::ConfigError("Invalid database URL format".to_string()));
        }

        let auth_parts: Vec<&str> = parts[0].split(':').collect();
        if auth_parts.len() != 2 {
            return Err(crate::StatsError::ConfigError("Invalid auth format in database URL".to_string()));
        }

        let host_parts: Vec<&str> = parts[1].split('/').collect();
        if host_parts.len() != 2 {
            return Err(crate::StatsError::ConfigError("Invalid host/database format in database URL".to_string()));
        }

        let host_port: Vec<&str> = host_parts[0].split(':').collect();
        let host = host_port[0].to_string();
        let port = if host_port.len() > 1 {
            host_port[1].parse().map_err(|_| crate::StatsError::ConfigError("Invalid port number".to_string()))?
        } else {
            5432
        };

        Ok(Self {
            host,
            port,
            database: host_parts[1].to_string(),
            username: auth_parts[0].to_string(),
            password: auth_parts[1].to_string(),
        })
    }
}
