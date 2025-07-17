//! Configuration-related data structures

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Create a new database configuration with defaults
    pub fn new(url: String) -> Self {
        Self { 
            url, 
            max_connections: 10 
        }
    }

    /// Set the maximum number of connections
    pub fn with_max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Get the database URL
    pub fn url(&self) -> &str {
        &self.url
    }
}
