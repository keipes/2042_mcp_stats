//! Stats client for querying weapon data

use tracing::{info, debug};
use crate::Result;
use crate::models::{DatabaseConfig, Weapon};
use crate::database::DatabaseManager;

/// Client for querying weapon statistics
pub struct StatsClient {
    db_manager: DatabaseManager,
}

impl StatsClient {
    /// Create a new stats client
    pub async fn new() -> Result<Self> {
        let config = DatabaseConfig::from_env()?;
        let db_manager = DatabaseManager::new(&config).await?;
        
        // Test the connection
        db_manager.test_connection().await?;
        
        info!("StatsClient initialized successfully");
        Ok(Self { db_manager })
    }

    /// Create a new stats client with custom configuration
    pub async fn with_config(config: &DatabaseConfig) -> Result<Self> {
        let db_manager = DatabaseManager::new(config).await?;
        db_manager.test_connection().await?;
        
        info!("StatsClient initialized with custom config");
        Ok(Self { db_manager })
    }

    /// Get weapons by category (placeholder implementation)
    pub async fn weapons_by_category(&self, _category_name: &str) -> Result<Vec<Weapon>> {
        debug!("Querying weapons by category");
        // Implementation will come in Phase 1.6
        todo!("Query implementation will be added in Phase 1.6")
    }

    /// Get a reference to the database manager
    pub fn database_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}
