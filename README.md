# Battlefield 2042 Weapon Statistics Library

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

#### Analytical Queries

```rust
// Find optimal configs for specific engagement scenarios
client.optimal_configs_for_range(
    filters: &ConfigFilters,
    range: i16,
    sort_by: SortCriteria,
    limit: Option<i32>
) -> impl Stream<Item = Result<OptimalConfig, StatsError>>

// Compare weapons across multiple metrics
client.weapon_comparison(
    weapon_names: &[&str],
    metrics: &[ComparisonMetric],
    ranges: &[i16]
) -> impl Stream<Item = Result<WeaponComparison, StatsError>>

// Time-to-kill analysis at various ranges
client.ttk_analysis(
    filters: &ConfigFilters,
    target_health: f64,
    ranges: &[i16]
) -> impl Stream<Item = Result<TtkAnalysis, StatsError>>

// Damage efficiency ranking (damage per shot vs rate of fire trade-offs)
client.damage_efficiency_ranking(
    category: Option<&str>,
    range: i16,
    weight_damage: f64,
    weight_rpm: f64
) -> impl Stream<Item = Result<EfficiencyRanking, StatsError>>

// Statistical aggregations across categories
client.category_statistics(
    metrics: &[StatMetric]
) -> impl Stream<Item = Result<CategoryStats, StatsError>>
```

#### Query Builder Pattern

```rust
// Flexible query builder for complex scenarios
let results = client
    .query_builder()
    .category("Assault Rifles")
    .min_damage_at_range(25.0, 50)
    .max_reload_time(2.5)
    .sort_by(SortBy::DamageDesc)
    .limit(10)
    .execute()
    .await?;
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

The binary provides database management commands and analytical queries:

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

### Analytical Commands

```bash
# Find best weapons for specific scenarios
bf2042-stats analyze best-at-range --category "Assault Rifles" --range 100 --limit 5

# Compare specific weapons
bf2042-stats compare --weapons "AK-24,M5A3,SFAR-M GL" --ranges "50,100,150"

# Time-to-kill analysis
bf2042-stats ttk --category "SMGs" --health 100 --ranges "10,25,50"

# Category statistics
bf2042-stats stats --category "Sniper Rifles" --metrics "damage,velocity,rpm"

# Export analysis results
bf2042-stats analyze damage-efficiency --output results.csv --format csv
```

### Query Examples

```bash
# Find highest DPS weapons at 75m
bf2042-stats query "
  SELECT w.weapon_name, c.config_id, calculated_dps
  FROM weapons_dps_at_range(75)
  WHERE category_name = 'LMGs'
  ORDER BY calculated_dps DESC
  LIMIT 5"

# Complex meta-analysis
bf2042-stats analyze meta --engagement-ranges "25,50,100" --weight-ttk 0.4 --weight-handling 0.3 --weight-range 0.3
```

### CLI Examples

```bash
# Complete database setup
bf2042-stats init

# Reset and repopulate
bf2042-stats clear && bf2042-stats schema && bf2042-stats populate

# Find meta weapons for competitive play
bf2042-stats analyze meta --ranges "50,100" --ttk-weight 0.5 --handling-weight 0.3 --versatility-weight 0.2

# Export weapon comparison data
bf2042-stats compare --weapons "all" --category "Assault Rifles" --output comparison.json
```

## Advanced Features

### Complex Query Support

The library supports sophisticated analytical queries through:

#### Computed Metrics

- **Time-to-Kill (TTK)**: Calculated based on damage, RPM, and target health
- **Damage Per Second (DPS)**: Real-world DPS accounting for reload times
- **Effective Range**: Optimal engagement distances for each configuration
- **Versatility Score**: Multi-range performance rating
- **Meta Rating**: Weighted scoring across multiple engagement scenarios

#### Query Optimization

- **View Materialization**: Pre-computed common analytical queries
- **Query Caching**: Intelligent caching of expensive calculations
- **Parallel Execution**: Multi-threaded analysis for large datasets
- **Incremental Updates**: Efficient recalculation when data changes

#### Statistical Functions

```rust
// Built-in statistical analysis
client.statistical_summary(
    query: &AnalyticalQuery,
    grouping: GroupBy
) -> Result<StatisticalSummary, StatsError>

// Correlation analysis between weapon attributes
client.attribute_correlation(
    attributes: &[WeaponAttribute],
    filters: &ConfigFilters
) -> Result<CorrelationMatrix, StatsError>

// Meta analysis across engagement scenarios
client.meta_analysis(
    scenarios: &[EngagementScenario],
    weights: &ScenarioWeights
) -> Result<MetaRanking, StatsError>
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

### Analytical Views and Functions

For complex queries, the schema includes materialized views and stored functions:

```sql
-- Pre-computed DPS at common ranges
CREATE MATERIALIZED VIEW weapons_dps_analysis AS
SELECT
    c.config_id,
    w.weapon_name,
    cat.category_name,
    calculate_dps_at_range(c.config_id, 25) as dps_25m,
    calculate_dps_at_range(c.config_id, 50) as dps_50m,
    calculate_dps_at_range(c.config_id, 100) as dps_100m,
    calculate_ttk_at_range(c.config_id, 100, 25) as ttk_25m_100hp,
    calculate_versatility_score(c.config_id) as versatility_score
FROM configurations c
JOIN weapons w ON c.weapon_id = w.weapon_id
JOIN categories cat ON w.category_id = cat.category_id;

-- Statistical functions for analysis
CREATE OR REPLACE FUNCTION calculate_meta_score(
    config_id INTEGER,
    scenario_weights DECIMAL[]
) RETURNS DECIMAL AS $$
-- Complex meta-analysis calculation
$$;
```

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
- **Query Optimization**: Materialized views for expensive analytical calculations
- **Parallel Processing**: Multi-threaded analysis for large computational workloads
- **Caching**: Intelligent caching of computed metrics and statistical results
- **Batch Operations**: Efficient bulk processing for complex analyses

## Thread Safety

The `StatsClient` is designed to be shared across threads:

- Uses `Arc<PgPool>` internally for thread-safe connection sharing
- All query methods are `async` and `Send + Sync`
- No mutable state in the client itself
