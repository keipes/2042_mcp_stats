#!/usr/bin/env bash

# Stop execution on error
set -e

# Start PostgreSQL container
lima nerdctl run --name bf2042-postgres \
  -e POSTGRES_DB=bf2042_stats \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -p 5432:5432 \
  -d postgres:16

# Set environment variables
export POSTGRES_HOST=localhost
export POSTGRES_PORT=5432
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=password
export POSTGRES_DB=bf2042_stats

# Initialize database with schema and data
cargo run --bin bf2042-stats init
