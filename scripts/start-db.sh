#!/bin/bash

# Start PostgreSQL container
echo "Starting PostgreSQL container..."
docker-compose up -d postgres

# Wait for database to be ready
echo "Waiting for database to be ready..."
timeout=30
while ! docker-compose exec postgres pg_isready -U postgres > /dev/null 2>&1; do
    timeout=$((timeout - 1))
    if [ $timeout -eq 0 ]; then
        echo "Database failed to start within 30 seconds"
        exit 1
    fi
    sleep 1
done

echo "PostgreSQL is ready!"
echo "Database URL: postgresql://postgres:password@localhost:5432/bf2042_stats"
