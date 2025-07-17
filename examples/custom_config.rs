//! Example: Initialize database with custom configuration

use bf2042_stats::{initialize_database_with_config, DatabaseConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create custom database configuration
    let config = DatabaseConfig::from_url(
        "postgresql://myuser:mypassword@localhost:5432/custom_db".to_string()
    );

    // Initialize database with custom configuration and embedded data
    let report = initialize_database_with_config(&config).await?;

    if report.is_valid {
        println!("✓ Database initialized successfully with custom config");
        println!("✓ Using embedded weapons data");
    } else {
        println!("⚠ Issues found during initialization:");
        for issue in &report.issues {
            println!("  - {}", issue);
        }
    }

    Ok(())
}
