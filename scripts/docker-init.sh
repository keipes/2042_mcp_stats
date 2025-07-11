#!/bin/bash
# PostgreSQL initialization script for Docker container
# This runs automatically when the container starts for the first time

set -e

echo "🔧 Initializing BF2042 Stats database schema..."

# Note: POSTGRES_DB, POSTGRES_USER, and POSTGRES_PASSWORD are already set by Docker
# The database 'bf2042_stats' is already created by the time this script runs

echo "📊 Database initialization complete!"
echo "✅ Ready for schema creation via 'cargo run -- init'"
