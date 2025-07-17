# Battlefield 2042 Weapon Statistics Library

🚧 **Current Status: Phase 1.1 Complete** 🚧

This project is currently under development following a phased implementation plan. **Phase 1.1** has been completed, providing the foundational architecture:

✅ **Completed in Phase 1.1:**

- Rust project initialized with all required dependencies
- Complete project structure with organized modules
- Error handling foundation with custom `StatsError` types
- Database models for weapons, categories, and configurations
- Database manager with connection handling
- Library-only architecture with initialization functions
- Thread-safe client architecture ready for future implementation

🔄 **Next Up: Phase 1.4-1.7**

- Database schema creation and SQL table definitions (✅ **COMPLETED**)
- JSON data parsing and population logic
- Basic query implementation
- End-to-end testing with PostgreSQL

See `PLAN.md` for the complete implementation roadmap.

---

A Rust library for querying Battlefield 2042 weapon statistics stored in a PostgreSQL database.

## Overview

This project provides:

- A thread-safe Rust library for querying weapon statistics
- Streaming query results for efficient data processing
- Database management utilities for schema migration and data population
- Simple initialization functions for database setup

## Architecture

### Database

- **Database**: PostgreSQL (runs in dev container)
- **Schema**: Normalized weapon statistics with performance indexes
- **Data Source**: `weapons.json` containing weapon configurations and damage dropoffs
- **Migration Strategy**: Drop and recreate database on schema/data changes

### Library Structure

- **Client**: Thread-safe connection pool with streaming query methods
- **Database Management**: Schema creation and data population from JSON
- **Error Handling**: Custom error types for connection failures and query errors
- **Async**: Built on `sqlx` for async database operations

## Dependencies

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid", "rust_decimal"] }
tokio = { version = "1.46", features = ["full"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "2.0"
tracing = "0.1"
tracing-subscriber = "0.3"
dotenvy = "0.15"
rust_decimal = { version = "1.35", features = ["serde"] }
```

## Environment Variables

The library reads database connection from the `DATABASE_URL` environment variable:

```env
# Complete database URL:
DATABASE_URL=postgresql://username:password@host:port/database_name

# Example for dev container:
DATABASE_URL=postgresql://postgres@localhost:5432/bf2042_stats
```

If `DATABASE_URL` is not set, the library defaults to:
`postgresql://postgres@localhost:5432/bf2042_stats`

## Development Setup

This project runs in a VS Code dev container with PostgreSQL pre-configured. To get started:

### Quick Setup

```bash
# Initialize database schema and populate with embedded data
cargo run --example initialize_db
```

This single command will:
- Connect to the dev container's PostgreSQL instance
- Create the `bf2042_stats` database if needed
- Set up the complete schema
- Populate with weapon data (embedded in the library)

### Manual Database Management

```bash
# Create database manually (if needed)
psql -h localhost -U postgres -c "CREATE DATABASE bf2042_stats;"

# Check PostgreSQL status
pg_isready -h localhost -p 5432 -U postgres
```

## Library Usage

### Client Initialization

```rust
use bf2042_stats::{StatsClient, StatsError};

#[tokio::main]
async fn main() -> Result<(), StatsError> {
    // Initialize the client with connection pool
    let client = StatsClient::new().await?;

    // Client can be cloned cheaply for use across threads
    let client_clone = client.clone();

    // Use client for queries...

    Ok(())
}
```

### Streaming Queries

```rust
use futures_util::StreamExt;

// Get all weapons in a category (returns immediately, executes when consumed)
let mut weapon_stream = client.weapons_by_category("Assault Rifles").await?;

while let Some(weapon) = weapon_stream.next().await {
    let weapon = weapon?;
    println!("Weapon: {} (ID: {})", weapon.weapon_name, weapon.weapon_id);
}
```

### Query Methods

#### Basic Queries

```rust
// Stream weapons by category
client.weapons_by_category(category: &str) -> impl Stream<Item = Result<Weapon, StatsError>>

// Stream weapon configurations with damage dropoffs
client.weapon_configs(weapon_name: &str) -> impl Stream<Item = Result<Configuration, StatsError>>

// Stream ammo statistics for weapon
client.weapon_ammo_stats(weapon_name: &str) -> impl Stream<Item = Result<WeaponAmmoStats, StatsError>>
```

### Database Initialization

```rust
use bf2042_stats::{initialize_database, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database with embedded weapons data (no external file needed)
    let report = initialize_database().await?;

    if report.is_valid {
        println!("✓ Database initialized successfully");
    } else {
        println!("⚠ Issues found during initialization:");
        for issue in &report.issues {
            println!("  - {}", issue);
        }
    }

    Ok(())
}
```

### Custom Configuration

```rust
use bf2042_stats::{initialize_database_with_config, DatabaseConfig, Result};

let config = DatabaseConfig::from_url(
    "postgresql://user:pass@localhost:5432/mydb".to_string()
);

// Using embedded data with custom config
let report = initialize_database_with_config(&config).await?;
```

### Schema-Only Creation

```rust
use bf2042_stats::{create_schema, Result};

// Create tables without populating data
create_schema().await?;
```

## Error Handling

Custom error types provide clear error categorization:

```rust
#[derive(thiserror::Error, Debug)]
pub enum StatsError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(#[from] sqlx::Error),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Data parsing error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

## CLI Tool

**Note: The CLI tool has been removed. All functionality is now available through the library API.**

Use the examples in the `examples/` directory to see how to initialize and manage the database:

```bash
# Initialize database
cargo run --example initialize_db

# Create schema only
cargo run --example create_schema

# Use custom configuration
cargo run --example custom_config
```

## Data Schema

The database normalizes weapon data into the following tables:

- `categories` - Weapon categories (Assault Rifles, SMGs, etc.)
- `weapons` - Individual weapons with category relationships
- `barrels` - Barrel attachments
- `ammo_types` - Ammunition types
- `weapon_ammo_stats` - Magazine size, reload times, headshot multipliers
- `configurations` - Weapon/barrel/ammo combinations with RPM and velocity
- `config_dropoffs` - Damage values at different ranges

See `SCHEMA.md` for detailed schema definition and example queries.

## Development

### Running Tests

```bash
# Run all tests (unit and integration)
cargo test

# Run only integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

### Database Management

```bash
# Initialize/reset database with embedded data (recommended)
cargo run --example initialize_db_embedded

# Initialize/reset database with external JSON file
cargo run --example initialize_db

# Create schema only (no data)
cargo run --example create_schema

# Use custom configuration
cargo run --example custom_config

# Prepare SQLx for offline compilation (if needed)
cargo sqlx prepare --merged
```

## Performance

- **Connection Pooling**: Shared `sqlx::PgPool` for efficient connection reuse
- **Streaming**: Query results stream without loading full datasets into memory
- **Indexes**: Optimized database indexes for common query patterns
- **Async**: Non-blocking database operations with Tokio runtime
- **Batch Operations**: Efficient bulk processing for data operations

## Thread Safety

The `StatsClient` is designed to be shared across threads:

- Uses `PgPool` directly (which is already thread-safe and cheaply cloneable)
- All query methods are `async` and `Send + Sync`
- No mutable state in the client itself
- Client can be cloned efficiently for use across threads
