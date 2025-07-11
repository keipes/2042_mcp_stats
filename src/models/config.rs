//! Configuration-related data structures

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    /// Get the database URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Create from DATABASE_URL environment variable
    pub fn from_env() -> crate::Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/bf2042_stats".to_string());

        Ok(Self { url })
    }

    /// Create from a specific URL
    pub fn from_url(url: String) -> Self {
        Self { url }
    }
}
