# Docker PostgreSQL Setup for BF2042 Stats

This guide helps you set up a PostgreSQL database using Docker for the BF2042 weapon stats project.

## Prerequisites

1. **Docker Desktop** - Download and install from [docker.com](https://www.docker.com/products/docker-desktop/)
2. **Docker Compose** - Included with Docker Desktop

## Quick Start

### 1. Start Docker Desktop

Make sure Docker Desktop is running before proceeding.

### 2. Start the Database

```powershell
# Windows PowerShell
.\scripts\start-db.ps1

# Or manually with docker-compose
docker-compose up -d postgres
```

### 3. Initialize the Database

```powershell
# Initialize schema and populate with data
.\scripts\setup-dev.ps1

# Or manually step by step
cargo run -- init
cargo run -- populate -i weapons.json
```

## Database Configuration

The Docker setup creates a PostgreSQL database with:

- **Host**: localhost
- **Port**: 5432
- **Database**: bf2042_stats
- **Username**: postgres
- **Password**: password
- **URL**: `postgresql://postgres:password@localhost:5432/bf2042_stats`

## Available Scripts

| Script                  | Description                        |
| ----------------------- | ---------------------------------- |
| `scripts/start-db.ps1`  | Start PostgreSQL container         |
| `scripts/stop-db.ps1`   | Stop PostgreSQL container          |
| `scripts/setup-dev.ps1` | Full development environment setup |

## Manual Docker Commands

```bash
# Start database
docker-compose up -d postgres

# View logs
docker-compose logs postgres

# Stop database
docker-compose down

# Remove all data (fresh start)
docker-compose down -v
```

## Connecting to Database

### Using psql

```bash
# Connect via Docker
docker-compose exec postgres psql -U postgres -d bf2042_stats

# Connect locally (if psql installed)
psql postgresql://postgres:password@localhost:5432/bf2042_stats
```

### Using your Rust application

The application will automatically connect using the `DATABASE_URL` from your `.env` file:

```
DATABASE_URL=postgresql://postgres:password@localhost:5432/bf2042_stats
```

## SQLx Offline Mode Setup

Once your database is running and populated:

```bash
# Generate SQLx query cache for offline compilation
cargo sqlx prepare

# Enable offline mode
echo "SQLX_OFFLINE=true" >> .env

# Now you can build without database connection
cargo build
```

## Troubleshooting

### Docker isn't running

- Make sure Docker Desktop is installed and running
- Check Docker Desktop system tray icon

### Port 5432 already in use

- Stop any existing PostgreSQL services
- Or change the port in `docker-compose.yml`

### Permission denied on scripts

```powershell
# Windows PowerShell - enable script execution
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Database connection refused

- Make sure the container is running: `docker-compose ps`
- Check logs: `docker-compose logs postgres`
- Wait a few seconds for PostgreSQL to fully start
