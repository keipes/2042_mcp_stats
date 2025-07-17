//! Integration tests for the BF2042 Stats library

use bf2042_stats::{models::DatabaseConfig, database::DatabaseManager, Result};
use std::env;

/// Test database configuration for integration tests
fn test_db_config(test_name: &str) -> DatabaseConfig {
    let test_db_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| format!("postgresql://postgres@localhost:5432/bf2042_stats_test_{}", test_name));
    
    DatabaseConfig::from_url(test_db_url)
}

/// Setup a clean test database
async fn setup_test_db(test_name: &str) -> Result<DatabaseManager> {
    let config = test_db_config(test_name);
    
    // Connect to main postgres database to create test database
    let main_config = DatabaseConfig::from_url("postgresql://postgres@localhost:5432/postgres".to_string());
    let main_manager = DatabaseManager::new(&main_config).await?;
    
    // Drop test database if it exists and create a new one
    let db_name = format!("bf2042_stats_test_{}", test_name);
    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .execute(main_manager.pool())
        .await
        .ok();
    
    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(main_manager.pool())
        .await?;
    
    // Now connect to the test database and create schema
    let manager = DatabaseManager::new(&config).await?;
    manager.create_schema().await?;
    
    Ok(manager)
}

#[tokio::test]
async fn test_database_connection() {
    let manager = setup_test_db("connection").await.expect("Failed to setup test database");
    
    // Test connection
    manager.test_connection().await.expect("Database connection test failed");
}

#[tokio::test]
async fn test_schema_creation() {
    let manager = setup_test_db("schema").await.expect("Failed to setup test database");
    
    // Schema should already be created by setup_test_db
    // Test that we can query the tables
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to query categories table");
    
    assert_eq!(result.0, 0); // Should be empty after reset
}

#[tokio::test]
async fn test_json_population() {
    let manager = setup_test_db("population").await.expect("Failed to setup test database");
    
    // Populate from embedded data
    manager.populate_from_embedded_data().await
        .expect("Failed to populate database from embedded data");
    
    // Verify data was populated
    let categories_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count categories");
    
    assert!(categories_count.0 > 0, "Categories should be populated");
    
    let weapons_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM weapons")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count weapons");
    
    assert!(weapons_count.0 > 0, "Weapons should be populated");
}

#[tokio::test]
async fn test_data_validation() {
    let manager = setup_test_db("validation").await.expect("Failed to setup test database");
    
    // Populate with embedded data
    manager.populate_from_embedded_data().await
        .expect("Failed to populate database");
    
    // Validate data integrity
    let report = manager.validate_data().await
        .expect("Failed to validate data");
    
    assert!(report.is_valid, "Data validation should pass: {:?}", report.issues);
    assert!(report.table_counts.len() > 0, "Should have table counts");
    
    // Check that all expected tables have data
    let expected_tables = ["categories", "weapons", "barrels", "ammo_types", "weapon_ammo_stats", "configurations", "config_dropoffs"];
    for table in &expected_tables {
        let count = report.table_counts.get(*table)
            .unwrap_or(&0);
        assert!(*count > 0, "Table {} should have data", table);
    }
}

#[tokio::test]
async fn test_database_reset() {
    let manager = setup_test_db("reset").await.expect("Failed to setup test database");
    
    // Populate database
    manager.populate_from_embedded_data().await
        .expect("Failed to populate database");
    
    // Verify data exists
    let count_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM weapons")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count weapons");
    assert!(count_before.0 > 0);
    
    // Reset database
    manager.reset_database().await
        .expect("Failed to reset database");
    
    // Verify data is gone
    let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM weapons")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count weapons after reset");
    assert_eq!(count_after.0, 0);
}

#[tokio::test]
async fn test_clear_data() {
    let manager = setup_test_db("clear").await.expect("Failed to setup test database");
    
    // Populate database
    manager.populate_from_embedded_data().await
        .expect("Failed to populate database");
    
    // Clear data
    manager.clear_data().await
        .expect("Failed to clear data");
    
    // Verify all tables are empty but still exist
    let tables = ["categories", "weapons", "barrels", "ammo_types", "weapon_ammo_stats", "configurations", "config_dropoffs"];
    
    for table in &tables {
        let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {}", table))
            .fetch_one(manager.pool())
            .await
            .unwrap_or_else(|_| panic!("Table {} should exist after clear_data", table));
        
        assert_eq!(count.0, 0, "Table {} should be empty after clear_data", table);
    }
}

#[tokio::test]
async fn test_embedded_data_population() {
    let manager = setup_test_db("embedded").await.expect("Failed to setup test database");
    
    // Populate from embedded data
    manager.populate_from_embedded_data().await
        .expect("Failed to populate database from embedded data");
    
    // Verify data was populated
    let categories_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count categories");
    
    assert!(categories_count.0 > 0, "Categories should be populated from embedded data");
    
    let weapons_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM weapons")
        .fetch_one(manager.pool())
        .await
        .expect("Failed to count weapons");
    
    assert!(weapons_count.0 > 0, "Weapons should be populated from embedded data");
}

#[tokio::test]
async fn test_config_from_env() {
    // Test default configuration
    env::remove_var("DATABASE_URL");
    let config = DatabaseConfig::from_env().expect("Failed to create config from env");
    assert!(config.url().contains("postgresql://"));
    assert!(config.url().contains("localhost:5432"));
    
    // Test with custom environment variable
    env::set_var("DATABASE_URL", "postgresql://custom@localhost:5432/custom_db");
    let config = DatabaseConfig::from_env().expect("Failed to create config from env");
    assert_eq!(config.url(), "postgresql://custom@localhost:5432/custom_db");
    
    // Clean up
    env::remove_var("DATABASE_URL");
}
