//! Battlefield 2042 Weapon Statistics Library
//!
//! This library provides access to weapon statistics and damage calculations
//! for Battlefield 2042, with PostgreSQL backend storage and streaming query support.

pub mod client;
pub mod database;
pub mod error;
pub mod models;

#[cfg(test)]
pub mod test_utils;

// Re-export main types for easier usage
pub use client::StatsClient;
pub use database::DatabaseManager;
pub use error::{Result, StatsError};
pub use models::{
    AmmoType, Barrel, Category, ConfigDropoff, Configuration, DatabaseConfig, ValidationReport, Weapon, WeaponAmmoStats,
};

/// Initialize the database with embedded weapons data
pub async fn initialize_database() -> Result<ValidationReport> {
    let config = DatabaseConfig::from_env()?;
    let db_manager = DatabaseManager::new(&config).await?;
    
    db_manager.reset_database().await?;
    db_manager.populate_from_embedded_data().await?;
    
    let report = db_manager.validate_data().await?;
    Ok(report)
}

/// Initialize the database with embedded data and custom configuration
pub async fn initialize_database_with_config(
    config: &DatabaseConfig,
) -> Result<ValidationReport> {
    let db_manager = DatabaseManager::new(config).await?;
    
    db_manager.reset_database().await?;
    db_manager.populate_from_embedded_data().await?;
    
    let report = db_manager.validate_data().await?;
    Ok(report)
}

/// Create schema only without data population
pub async fn create_schema() -> Result<()> {
    let config = DatabaseConfig::from_env()?;
    let db_manager = DatabaseManager::new(&config).await?;
    
    db_manager.create_schema().await?;
    Ok(())
}

/// Create schema with custom configuration
pub async fn create_schema_with_config(config: &DatabaseConfig) -> Result<()> {
    let db_manager = DatabaseManager::new(config).await?;
    
    db_manager.create_schema().await?;
    Ok(())
}
