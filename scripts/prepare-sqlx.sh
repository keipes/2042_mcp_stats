#!/bin/bash
# Script to prepare SQLx queries for offline compilation

# start docker
# apply schema to database

set -e

echo "Starting database if not running..."
if ! docker ps | grep -q postgres; then
    echo "Starting PostgreSQL container..."
    ./scripts/start-db.sh
    echo "Waiting for database to be ready..."
    sleep 5
else
    echo "Database is already running"
fi

docker exec -i bf2042_stats_db psql -U postgres -d bf2042_stats < schema.sql

echo "Preparing SQLx queries..."
cargo sqlx prepare --merged

echo "SQLx queries prepared successfully!"
echo "The .sqlx/ directory now contains all query metadata for offline compilation."
