//! Example: Initialize database using embedded weapons data

use bf2042_stats::{initialize_database_embedded, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    info!("Initializing database with embedded weapons data...");

    // Initialize database with embedded data (no external file needed)
    let report = initialize_database_embedded().await?;

    if report.is_valid {
        println!("✓ Database initialized successfully with embedded data");
        println!("✓ Data integrity: OK");
    } else {
        println!("⚠ Database initialized with issues:");
        for issue in &report.issues {
            println!("  - {}", issue);
        }
    }

    println!("\nTable counts:");
    for (table, count) in &report.table_counts {
        println!("  {}: {}", table, count);
    }

    println!("\n🎯 The weapons data is now embedded in the library!");
    println!("   No external files needed when using this crate.");

    Ok(())
}
