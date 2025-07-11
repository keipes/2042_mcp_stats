# Cross-platform database initialization script for Windows
# This script initializes the database schema and populates it with data

Write-Host "Initializing BF2042 Stats database..." -ForegroundColor Cyan

# Check if DATABASE_URL environment variable is set
if (-not $env:DATABASE_URL) {
    $env:DATABASE_URL = "postgresql://postgres:password@localhost:5432/bf2042_stats"
    Write-Host "Using default DATABASE_URL: $env:DATABASE_URL" -ForegroundColor Yellow
}

# Wait for database to be ready
Write-Host "Waiting for database to be ready..." -ForegroundColor Yellow
$timeout = 60
do {
    docker-compose exec postgres pg_isready -U postgres 2>$null | Out-Null
    if ($LASTEXITCODE -eq 0) {
        break
    }
    $timeout--
    if ($timeout -eq 0) {
        Write-Host "Database failed to start within 60 seconds" -ForegroundColor Red
        exit 1
    }
    Start-Sleep 1
} while ($timeout -gt 0)

Write-Host "Database is ready!" -ForegroundColor Green

# Initialize schema and populate with data
Write-Host "Initializing database (schema + data)..." -ForegroundColor Cyan
if (Test-Path "weapons.json") {
    cargo run -- init --force --file weapons.json
} else {
    Write-Host "weapons.json not found, using default" -ForegroundColor Yellow
    cargo run -- init --force
}

Write-Host "Database initialization complete!" -ForegroundColor Green
Write-Host "Database URL: $env:DATABASE_URL" -ForegroundColor Cyan
