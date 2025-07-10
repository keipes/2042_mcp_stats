# Implementation Plan for Battlefield 2042 Weapon Statistics Library

## Phase 1: Project Foundation & Vertical Slice (Core Architecture)

### 1.1 Project Setup ✅

- [x] Initialize Rust project with `cargo init --lib`
- [x] Configure `Cargo.toml` with required dependencies:
  - `sqlx` (PostgreSQL support, async runtime)
  - `tokio` (async runtime)
  - `serde` (JSON serialization)
  - `anyhow`/`thiserror` (error handling)
  - `clap` (CLI)
  - `tracing` (logging)
  - `dotenvy` (environment variables)
- [x] Set up project structure:
  ```
  src/
    lib.rs          # Library exports
    client.rs       # StatsClient implementation
    database/       # Database management
      mod.rs
      manager.rs    # DatabaseManager
      schema.rs     # Schema creation
      migration.rs  # Data population
    models/         # Data structures
      mod.rs
      weapon.rs     # Weapon-related structs
      config.rs     # Configuration structs
    error.rs        # Custom error types
    main.rs         # CLI binary
  ```

### 1.2 Error Handling Foundation ✅

- [x] Define `StatsError` enum with variants:
  - `ConnectionFailed(sqlx::Error)`
  - `QueryFailed(String)`
  - `ParseError(serde_json::Error)`
  - `IoError(std::io::Error)`
  - `ConfigError(String)`
- [x] Implement `From` traits for automatic error conversion
- [x] Add `Result<T>` type alias

### 1.3 Database Models (Minimal Set) ✅

- [x] Create core data structures:
  - `Category` struct
  - `Weapon` struct
  - `WeaponConfig` struct (includes barrel, ammo, stats)
  - `DamageDropoff` struct
- [x] Implement `serde` derives for JSON parsing
- [x] Add `sqlx::FromRow` derives for database mapping

### 1.4 Database Schema Creation

- [ ] Implement `DatabaseManager::create_schema()` method
- [ ] Create SQL for core tables:
  - `categories` table
  - `weapons` table
  - `barrels` table
  - `ammo_types` table
- [ ] Add primary keys and basic indexes
- [ ] Test schema creation with Docker PostgreSQL

### 1.5 Basic JSON Data Population

- [ ] Implement `DatabaseManager::populate_from_json()` method
- [ ] Parse `weapons.json` structure:
  - Extract unique categories
  - Extract unique barrel types
  - Extract unique ammo types
  - Extract weapons with category relationships
- [ ] Insert data into core tables (categories, weapons, barrels, ammo_types)
- [ ] Test end-to-end data flow: JSON → Database

### 1.6 Minimal Client Implementation ✅

- [x] Create `StatsClient` struct with `PgPool`
- [x] Implement `StatsClient::new()` for connection initialization
- [x] Add one basic query method: `weapons_by_category()` (placeholder)
- [x] Return simple `Vec<Weapon>` (streaming comes later)
- [x] Test query functionality (placeholder implementation)

### 1.7 Basic CLI ✅

- [x] Implement CLI with `clap`
- [x] Add commands:
  - `init` - Create schema and populate data (placeholder)
  - `status` - Show database connection status
- [x] Test complete workflow: CLI → Database → Query (with placeholders)

## Phase 2: Enhanced Database Schema & Streaming

### 2.1 Complete Database Schema

- [ ] Add remaining tables:
  - `configurations` (weapon/barrel/ammo combinations)
  - `weapon_ammo_stats` (magazine, reload times, multipliers)
  - `config_dropoffs` (damage at different ranges)
- [ ] Implement all foreign key relationships
- [ ] Add performance indexes per SCHEMA.md
- [ ] Add database constraints and validations

### 2.2 Complete Data Population

- [ ] Extend JSON parsing to handle all data:
  - Weapon configurations (barrel + ammo combinations)
  - Ammo statistics (magazine size, reload times, etc.)
  - Damage dropoffs per configuration
- [ ] Implement batch insert operations for efficiency
- [ ] Add data validation during import
- [ ] Handle duplicate detection and conflicts

### 2.3 Streaming Query Implementation

- [ ] Convert query methods to return `impl Stream`
- [ ] Implement streaming for:
  - `weapons_by_category()`
  - `weapon_configs()` (weapon configurations with damage dropoffs)
  - `weapon_ammo_stats()`
- [ ] Use `sqlx::query_as!` with proper mapping
- [ ] Add error handling within streams

### 2.4 Enhanced Client Methods

- [ ] Add complex query methods:
  - `damage_at_range()` - Get effective damage for configs at specific range
  - `best_configs_in_category()` - Top performing configs by category
  - `weapon_details()` - Complete weapon information
- [ ] Implement proper SQL queries per SCHEMA.md examples
- [ ] Add query parameter validation

## Phase 3: Advanced Features & CLI Enhancement

### 3.1 Database Management Features

- [ ] Implement `DatabaseManager::reset_database()` - Drop and recreate
- [ ] Add `clear_data()` - Remove data only, keep schema
- [ ] Add `validate_data()` - Check data integrity
- [ ] Implement transaction support for atomic operations

### 3.2 Enhanced CLI Commands

- [ ] Add comprehensive CLI commands:
  - `clear` - Clear all data
  - `schema` - Apply schema only
  - `populate` - Populate data with optional file path
  - `validate` - Validate database state
- [ ] Add command-line options:
  - `--force` for destructive operations
  - `--file` for custom JSON file paths
  - `--verbose` for detailed output
- [ ] Implement proper error messages and user feedback

### 3.3 Configuration & Environment

- [ ] Support multiple database connection methods:
  - `DATABASE_URL` environment variable
  - Individual components (`POSTGRES_HOST`, `POSTGRES_PORT`, etc.)
- [ ] Add configuration validation
- [ ] Implement connection retry logic
- [ ] Add connection pooling configuration options

### 3.4 Performance Optimization

- [ ] Implement connection pooling best practices
- [ ] Add query result caching where appropriate
- [ ] Optimize batch operations for large datasets
- [ ] Add query performance monitoring
- [ ] Implement proper index usage validation

## Phase 4: Production Readiness

### 4.1 Comprehensive Error Handling

- [ ] Add context to all error types
- [ ] Implement proper error propagation in streams
- [ ] Add detailed error messages for common failures
- [ ] Implement graceful degradation for connection issues

### 4.2 Logging & Observability

- [ ] Add structured logging with `tracing`
- [ ] Log database operations and performance metrics
- [ ] Add debug logging for troubleshooting
- [ ] Implement proper log levels

### 4.3 Thread Safety & Concurrency

- [ ] Verify thread safety of all components
- [ ] Add concurrent query testing
- [ ] Implement proper resource cleanup
- [ ] Add stress testing for connection pool

### 4.4 Documentation & Examples

- [ ] Complete inline documentation for all public APIs
- [ ] Add comprehensive README examples
- [ ] Create usage examples for common patterns
- [ ] Document performance characteristics
- [ ] Add troubleshooting guide

## Development Workflow

1. **Start each phase** by implementing the minimal viable functionality
2. **Test thoroughly** at each step with Docker PostgreSQL container
3. **Validate against real data** using the provided `weapons.json`
4. **Iterate quickly** on the vertical slice before moving to next phase

## Success Criteria for Phase 1 (Vertical Slice)

- [ ] PostgreSQL container runs and accepts connections
- [ ] Database schema creates successfully
- [ ] JSON data parses and populates basic tables
- [ ] CLI can initialize database and show status
- [ ] Client can connect and perform basic weapon queries
- [ ] End-to-end flow works: JSON → Database → Query → Results

This plan ensures a working end-to-end system from the first phase, allowing for immediate testing and validation of the architecture decisions.
