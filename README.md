# Battlefield 2042 Weapon Statistics Library

🚧 **Current Status: Phase 1.1 Complete** 🚧

This project is currently under development following a phased implementation plan. **Phase 1.1** has been completed, providing the foundational architecture:

✅ **Completed in Phase 1.1:**

- Rust project initialized with all required dependencies
- Complete project structure with organized modules
- Error handling foundation with custom `StatsError` types
- Database models for weapons, categories, and configurations
- Database manager with connection handling
- Basic CLI with `init` and `status` commands
- Thread-safe client architecture ready for future implementation

🔄 **Next Up: Phase 1.4-1.7**

- Database schema creation and SQL table definitions
- JSON data parsing and population logic
- Basic query implementation
- End-to-end testing with Docker PostgreSQL

See `PLAN.md` for the complete implementation roadmap.

---

A Rust library and CLI tool for querying Battlefield 2042 weapon statistics stored in a PostgreSQL database.

## Overview

This project provides:

- A thread-safe Rust library for querying weapon statistics
- Streaming query results for efficient data processing
- Database management utilities for schema migration and data population
- A CLI tool for database initialization and management

## Architecture

### Database

- **Database**: PostgreSQL (containerized)
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
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid", "decimal"] }
tokio = { version = "1.42", features = ["full"] }
tokio-stream = "0.1"
futures-util = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "2.0"
clap = { version = "4.5", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = "0.3"
dotenvy = "0.15"
rust_decimal = { version = "1.36", features = ["serde"] }
ordered-float = "4.3"

[dev-dependencies]
testcontainers = "0.23"
tokio-test = "0.4"
criterion = "0.6"
proptest = "1.6"
```

## Environment Variables

The library reads database connection parameters from environment variables:

```env
DATABASE_URL=postgresql://username:password@localhost:5432/bf2042_stats
# Or individual components:
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USER=username
POSTGRES_PASSWORD=password
POSTGRES_DB=bf2042_stats
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
    println!("Weapon: {} (ID: {})", weapon.name, weapon.id);
}
```

### Query Methods

#### Basic Queries

```rust
// Stream weapons by category
client.weapons_by_category(category: &str) -> impl Stream<Item = Result<Weapon, StatsError>>

// Stream weapon configurations with damage dropoffs
client.weapon_configs(weapon_name: &str) -> impl Stream<Item = Result<WeaponConfig, StatsError>>

// Stream ammo statistics for weapon
client.weapon_ammo_stats(weapon_name: &str) -> impl Stream<Item = Result<AmmoStats, StatsError>>
```

### Database Management

```rust
use bf2042_stats::database::{DatabaseManager, Migration};

let db_manager = DatabaseManager::new().await?;

// Drop existing data and recreate schema
db_manager.reset_database().await?;

// Populate from weapons.json
db_manager.populate_from_json("weapons.json").await?;
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

The binary provides database management commands:

### Installation

```bash
cargo install --path .
```

### Database Management

```bash
# Initialize database with schema and data
bf2042-stats init

# Clear all data
bf2042-stats clear

# Apply schema only
bf2042-stats schema

# Populate data from JSON
bf2042-stats populate [--file weapons.json]

# Show database status
bf2042-stats status
```

### CLI Examples

```bash
# Complete database setup
bf2042-stats init

# Reset and repopulate
bf2042-stats clear && bf2042-stats schema && bf2042-stats populate
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

### Running with Docker

```bash
# Start PostgreSQL container
docker run --name bf2042-postgres \
  -e POSTGRES_DB=bf2042_stats \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  -d postgres:16

# Set environment variables
export DATABASE_URL=postgresql://postgres:password@localhost:5432/bf2042_stats

# Initialize database
cargo run --bin bf2042-stats init
```

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests (requires Docker)
cargo test --features integration-tests
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
