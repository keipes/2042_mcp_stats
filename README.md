# MCP Stats - Battlefield 2042 Weapons Database

A Rust library and binary for querying Battlefield 2042 weapons statistics with Model Context Protocol (MCP) Tool integration.

## Overview

This project provides:

- **Rust Library**: Methods for querying weapons data via SQLx with MCP Tool definitions
- **Binary**: Database schema application and data population for PostgreSQL
- **Data Source**: Battlefield 2042 weapons statistics from `weapons.json`

## Features

### Library (`lib.rs`)

- `WeaponsDb` struct that encapsulates database connection and operations
- Thread-safe design - `WeaponsDb` implements `Clone` and can be shared across async tasks
- SQLx-based database querying methods on the main struct
- Synchronous methods returning lazy async streams - no `.await` needed on method calls
- MCP Tool definitions for external integration
- Database population methods from JSON source data
- Custom error types for robust error handling
- Builder pattern for connection configuration

### Binary (`main.rs`)

- PostgreSQL schema application
- Database population from `weapons.json`
- Connection parameter CLI arguments (`--host`, `--port`, `--user`, `--password`, `--database`)
- Database recreation on schema/data changes

## Database Schema

The database consists of 7 normalized tables:

- `categories` - Weapon categories (Sidearms, Assault Rifles, etc.)
- `weapons` - Individual weapons with category relationships
- `barrels` - Barrel types and modifications
- `ammo_types` - Ammunition types and variants
- `weapon_ammo_stats` - Magazine size, reload times, headshot multipliers per weapon/ammo combination
- `configurations` - Weapon/barrel/ammo combinations with velocity and RPM stats
- `config_dropoffs` - Damage values at different ranges for each configuration

## Data Structure

Source data in `weapons.json` contains:

- Weapon categories with nested weapons
- Per-weapon statistics by barrel type and ammo type
- Damage dropoff curves by range
- Ammo-specific stats (magazine size, reload times, headshot multipliers)
- RPM data (single, burst, auto) and velocity

## MCP Tool Definitions

The library exposes weapons database queries as MCP tools for external integration:

- Get weapons by category
- Get weapon configurations and stats
- Get damage dropoff data
- Query optimal configurations by criteria

## Dependencies

- **SQLx**: Database operations and migrations
- **Tokio**: Async runtime
- **Serde**: JSON serialization/deserialization
- **Clap**: CLI argument parsing
- **Futures**: For Stream trait and async utilities
- **Tokio-stream**: Stream utilities and adapters

## Database Requirements

- PostgreSQL (recommend using Docker: `postgres:latest`)
- Connection parameters configurable via CLI arguments
- Automatic schema recreation on changes

## Error Handling

Custom error types for:

- Database connection failures
- Schema application errors
- Data parsing and validation errors
- MCP tool execution errors

## Usage

### As Library

```rust
use mcp_stats::WeaponsDb;
use futures::StreamExt;

// Initialize database connection and populate
let db = WeaponsDb::connect(host, port, user, password, database).await?;
db.populate_from_json("weapons.json").await?;

// Methods are synchronous - return streams immediately (lazy evaluation)
let weapons_stream = db.weapons_by_category("Assault Rifles");
let weapons: Vec<_> = weapons_stream.try_collect().await?;

// Process results as they arrive (memory efficient)
let mut dropoff_stream = db.weapon_dropoffs("AK-24");
while let Some(dropoff) = dropoff_stream.try_next().await? {
    println!("Range: {}, Damage: {}", dropoff.range, dropoff.damage);
}

// Chain operations fluently without intermediate awaits
let high_velocity_configs: Vec<_> = db.all_configurations()
    .try_filter(|config| futures::future::ready(config.velocity > 800))
    .try_collect()
    .await?;

// Get first match only
let best_weapon = db.best_configs_at_range("Assault Rifles", 100)
    .try_next()
    .await?;

// Compose multiple streams
let combined_weapons = db.weapons_by_category("Assault Rifles")
    .chain(db.weapons_by_category("SMG"))
    .try_collect()
    .await?;

// Thread-safe: can be cloned and shared across async tasks
let db_clone = db.clone();
tokio::spawn(async move {
    let results: Vec<_> = db_clone.weapons_by_category("Sidearms")
        .try_collect()
        .await?;
    // Process results...
});
```

### As Binary

```bash
# Apply schema and populate data
./mcp_stats --host localhost --port 5432 --user postgres --password secret --database bf2042
```

## Development Status

- [x] Project definition and README
- [ ] Database schema implementation
- [ ] SQLx migrations
- [ ] JSON data parsing and population
- [ ] Core query methods
- [ ] MCP Tool definitions
- [ ] CLI binary implementation
- [ ] Error type definitions
- [ ] Testing and documentation
