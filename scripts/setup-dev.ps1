# Development setup script for BF2042 Stats
Write-Host "Setting up BF2042 Stats development environment..."

# Check if Docker is running
$dockerRunning = docker info 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Docker is not running. Please start Docker Desktop." -ForegroundColor Red
    exit 1
}

# Start PostgreSQL
Write-Host "Starting PostgreSQL container..."
docker-compose up -d postgres

# Wait for database
Write-Host "Waiting for database to be ready..."
$timeout = 30
do {
    $result = docker-compose exec postgres pg_isready -U postgres 2>$null
    if ($LASTEXITCODE -eq 0) {
        break
    }
    $timeout--
    if ($timeout -eq 0) {
        Write-Host "Database failed to start within 30 seconds" -ForegroundColor Red
        exit 1
    }
    Start-Sleep 1
} while ($timeout -gt 0)

Write-Host "Database is ready!" -ForegroundColor Green

# Initialize database schema
Write-Host "Initializing database schema..."
cargo run -- init

# Populate with sample data
Write-Host "Populating database with weapon data..."
cargo run -- populate -i weapons.json

Write-Host "Development environment setup complete!" -ForegroundColor Green
Write-Host "Database URL: postgresql://postgres:password@localhost:5432/bf2042_stats" -ForegroundColor Cyan
Write-Host ""
Write-Host "Useful commands:"
Write-Host "  cargo run -- status        # Check database status"
Write-Host "  cargo run -- validate      # Validate data integrity"
Write-Host "  cargo sqlx prepare         # Prepare SQLx offline queries"
Write-Host "  ./scripts/stop-db.ps1      # Stop the database"
