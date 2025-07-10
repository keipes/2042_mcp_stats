//! CLI binary for Battlefield 2042 weapon statistics

use clap::{Parser, Subcommand};
use tracing::{info, error, Level};
use tracing_subscriber;

use bf2042_stats::{StatsClient, Result, StatsError};

#[derive(Parser)]
#[command(name = "bf2042-stats")]
#[command(about = "Battlefield 2042 weapon statistics CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize database schema and populate with data
    Init {
        /// Force reinitialize even if database exists
        #[arg(short, long)]
        force: bool,
        
        /// Path to weapons JSON file
        #[arg(short, long, default_value = "weapons.json")]
        file: String,
    },
    /// Show database connection status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    match cli.command {
        Commands::Init { force, file } => {
            info!("Initializing database...");
            
            if force {
                info!("Force flag enabled - will recreate database");
            }
            
            // For now, just test the connection
            let client = StatsClient::new().await?;
            info!("Database connection successful");
            
            // Schema creation and data population will be implemented in later phases
            info!("Schema creation and data population coming in Phase 1.4-1.5");
            info!("Would use JSON file: {}", file);
            
            Ok(())
        }
        Commands::Status => {
            info!("Checking database status...");
            
            match StatsClient::new().await {
                Ok(_client) => {
                    println!("✓ Database connection: OK");
                    println!("✓ Configuration: Valid");
                    Ok(())
                }
                Err(e) => {
                    error!("Database connection failed: {}", e);
                    println!("✗ Database connection: FAILED");
                    println!("  Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
