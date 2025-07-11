#!/bin/bash
# Cross-platform database initialization script
# This script initializes the database schema and populates it with data

set -e

echo "🔧 Initializing BF2042 Stats database..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    export DATABASE_URL="postgresql://postgres:password@localhost:5432/bf2042_stats"
    echo "📡 Using default DATABASE_URL: $DATABASE_URL"
fi

# Wait for database to be ready
echo "⏳ Waiting for database to be ready..."
timeout=60
while ! pg_isready -d "$DATABASE_URL" > /dev/null 2>&1; do
    timeout=$((timeout - 1))
    if [ $timeout -eq 0 ]; then
        echo "❌ Database failed to start within 60 seconds"
        exit 1
    fi
    sleep 1
done

echo "✅ Database is ready!"

# Initialize schema and populate with data
echo "📊 Initializing database (schema + data)..."
if [ -f "weapons.json" ]; then
    echo "🔫 Using weapons.json data file"
    cargo run -- init --force --file weapons.json
else
    echo "⚠️  weapons.json not found, using default"
    cargo run -- init --force
fi

echo "🎉 Database initialization complete!"
echo "📡 Database URL: $DATABASE_URL"
