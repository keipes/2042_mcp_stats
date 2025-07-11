# Start PostgreSQL container
Write-Host "Starting PostgreSQL container..."
docker-compose up -d postgres

# Wait for database to be ready
Write-Host "Waiting for database to be ready..."
$timeout = 30
do {
    $result = docker-compose exec postgres pg_isready -U postgres 2>$null
    if ($LASTEXITCODE -eq 0) {
        break
    }
    $timeout--
    if ($timeout -eq 0) {
        Write-Host "Database failed to start within 30 seconds"
        exit 1
    }
    Start-Sleep 1
} while ($timeout -gt 0)

Write-Host "PostgreSQL is ready!"
Write-Host "Database URL: postgresql://postgres:password@localhost:5432/bf2042_stats"
