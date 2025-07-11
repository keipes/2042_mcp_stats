//! CLI binary for Battlefield 2042 weapon statistics

use clap::{Parser, Subcommand};
use tracing::{info, error, Level};
use tracing_subscriber;

use bf2042_stats::{StatsClient, Result};

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
    /// Initialize database (reset schema and populate with data)
    Init {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
        
        /// Path to weapons JSON file
        #[arg(short = 'i', long, default_value = "weapons.json")]
        file: String,
    },
    /// Run demo queries to showcase client functionality
    Demo,
}

/// Demonstrate client query functionality with real data
async fn demo_client_queries() -> Result<()> {
    println!("🎯 BF2042 Stats Client Demo");
    println!("==========================");
    println!();

    // Initialize client
    println!("📡 Connecting to database...");
    let client = StatsClient::new().await?;
    println!("✓ Connected successfully");
    println!();

    // Demo 1: List weapons in Assault Rifles category
    println!("🔫 Demo 1: Weapons in Assault Rifles category");
    println!("{}", "-".repeat(45));
    let weapons = client.weapons_by_category("Assault Rifles").await?;
    
    if weapons.is_empty() {
        println!("❌ No weapons found in Assault Rifles category");
        return Ok(());
    }
    
    println!("Found {} weapons (showing first 5):", weapons.len());
    for (i, weapon) in weapons.iter().take(5).enumerate() {
        println!("  {}. {}", i + 1, weapon.weapon_name);
    }
    println!();

    // Demo 2: Get details for first weapon
    let demo_weapon = &weapons[0];
    println!("🔍 Demo 2: Details for '{}'", demo_weapon.weapon_name);
    println!("{}", "-".repeat(45));
    
    let details = client.weapon_details(&demo_weapon.weapon_name).await?;
    println!("Configurations: {}", details.configurations.len());
    println!("Ammo types: {}", details.ammo_stats.len());
    
    // Show first ammo type stats
    if let Some(ammo) = details.ammo_stats.first() {
        println!("Sample ammo ({}):", ammo.ammo_type_name);
        println!("  Magazine: {} rounds", ammo.magazine_size);
        println!("  Headshot multiplier: {:.1}x", ammo.headshot_multiplier);
    }
    println!();

    // Demo 3: Damage at 100m
    println!("💥 Demo 3: Damage at 100m for '{}'", demo_weapon.weapon_name);
    println!("{}", "-".repeat(45));
    
    let damage_configs = client.damage_at_range(&demo_weapon.weapon_name, 100).await?;
    
    if damage_configs.is_empty() {
        println!("❌ No damage data at 100m");
    } else {
        println!("Best configuration:");
        let best = &damage_configs[0];
        println!("  {} + {}", best.barrel_name, best.ammo_type_name);
        println!("  Damage: {} at {}m", best.damage, best.effective_range);
        println!("  Velocity: {} m/s", best.velocity);
    }
    println!();

    // Demo 4: Top 3 configurations in category at 29m
    println!("🏆 Demo 4: Top Assault Rifle configs at 29m");
    println!("{}", "-".repeat(45));
    
    let best_configs = client.best_configs_in_category("Assault Rifles", 29, 20).await?;
    
    for (i, config) in best_configs.iter().enumerate() {
        println!("  {}. {} ({} + {})", 
            i + 1, config.weapon_name, config.barrel_name, config.ammo_type_name);
        println!("     {} damage, {} rounds", config.damage, config.magazine_size);
    }
    
    println!();
    println!("✅ Demo completed - all queries successful!");
    
    Ok(())
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
            
            if !force {
                println!("This will completely reset the database and reload all data.");
                println!("ALL EXISTING DATA WILL BE PERMANENTLY LOST!");
                print!("Are you sure you want to continue? (y/N): ");
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
                    println!("Operation cancelled.");
                    return Ok(());
                }
            }
            
            let client = StatsClient::new().await?;
            let db_manager = client.database_manager();
            
            info!("Resetting database schema...");
            db_manager.reset_database().await?;
            
            info!("Populating data from JSON file: {}", file);
            db_manager.populate_from_json(&file).await?;
            
            // Show validation report
            let report = db_manager.validate_data().await?;
            
            println!("✓ Database reset and initialized successfully");
            println!("✓ Schema created");
            println!("✓ Data populated from {}", file);
            
            if report.is_valid {
                println!("✓ Data integrity: OK");
            } else {
                println!("⚠ Data integrity: ISSUES FOUND");
                for issue in &report.issues {
                    println!("  - {}", issue);
                }
            }
            
            println!("\nTable counts:");
            for (table, count) in &report.table_counts {
                println!("  {}: {}", table, count);
            }
            
            Ok(())
        }
        Commands::Demo => {
            info!("Running demo queries...");
            
            match demo_client_queries().await {
                Ok(_) => {
                    println!("\n✓ Demo completed successfully!");
                    println!("All client queries executed without errors.");
                }
                Err(e) => {
                    error!("Demo failed: {}", e);
                    println!("✗ Demo failed: {}", e);
                    println!("Make sure the database is initialized with: cargo run -- init");
                    std::process::exit(1);
                }
            }
            
            Ok(())
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
