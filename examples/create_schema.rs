//! Example: Create database schema only (without data)

use bf2042_stats::{create_schema, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    info!("Creating database schema...");

    // Create schema without populating data
    create_schema().await?;

    println!("✓ Database schema created successfully");
    println!("Ready for data population using initialize_database()");

    Ok(())
}
