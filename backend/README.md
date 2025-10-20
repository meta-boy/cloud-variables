# Cloud Variables Backend

A Rust-based backend service for managing cloud variables with PostgreSQL, Redis, and comprehensive tier-based user management.

## Features

- **Database Migration System**: Fully automated database migrations using sqlx
- **Tier-Based Access Control**: Free, Basic, Pro, and Enterprise tiers with customizable limits
- **User Management**: Complete user authentication and authorization
- **API Key Management**: Secure API key generation and validation
- **Usage Tracking**: Track API calls and storage usage per user
- **Promotion History**: Audit trail for tier changes

## Prerequisites

- Rust 1.70+ (edition 2021)
- Docker & Docker Compose
- PostgreSQL 16
- Redis 7

## Quick Start

### 1. Setup Development Environment

```bash
# Run the automated setup script
./scripts/setup_dev.sh
```

This script will:
- Create `.env` file from template
- Start PostgreSQL and Redis containers
- Create necessary directories
- Install sqlx-cli if needed

### 2. Configure Environment

Edit `.env` file with your configuration:

```bash
# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/cloud_variables
DATABASE_MAX_CONNECTIONS=10

# Redis
REDIS_URL=redis://localhost:6379

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# JWT
JWT_SECRET=your-secret-key-change-in-production
JWT_EXPIRATION_HOURS=24

# Storage
STORAGE_TYPE=filesystem
STORAGE_PATH=./data/variables

# Admin
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=change-this-password

# Logging
RUST_LOG=info,cloud_variables=debug
ENVIRONMENT=development
```

### 3. Run Migrations

```bash
# Using the migration script
./scripts/run_migrations.sh

# Or run the application (migrations run automatically)
cargo run
```

### 4. Start the Application

```bash
cargo run
```

## Database Migrations

The project uses sqlx for database migrations. All migrations are located in the `migrations/` directory:

1. **20250101000001_create_users.sql** - Creates users table
2. **20250101000002_create_tiers.sql** - Creates tiers table
3. **20250101000003_create_api_keys.sql** - Creates API keys table
4. **20250101000004_create_variables.sql** - Creates variables table
5. **20250101000005_create_usage_stats.sql** - Creates usage statistics table
6. **20250101000006_create_promotion_history.sql** - Creates promotion history table
7. **20250101000007_seed_default_tiers.sql** - Seeds default tier data

### Running Migrations Manually

```bash
# Using sqlx-cli
sqlx migrate run

# Using the application
cargo run
```

### Creating New Migrations

```bash
sqlx migrate add <migration_name>
```

## Project Structure

```
backend/
├── migrations/                    # Database migrations
├── scripts/                       # Utility scripts
│   ├── setup_dev.sh              # Development environment setup
│   ├── run_migrations.sh         # Run database migrations
│   └── create_admin.sh           # Create admin user
├── src/
│   ├── db/                       # Database connection & pool
│   │   ├── mod.rs
│   │   └── pool.rs
│   ├── lib.rs                    # Library exports
│   └── main.rs                   # Application entry point
├── .env.example                  # Environment variables template
├── docker-compose.yml            # Docker services configuration
└── Cargo.toml                    # Rust dependencies
```

## Default Tiers

The system comes with four pre-configured tiers:

| Tier       | Variables | Max Size | API Calls/Day | API Keys | Price/Month |
|------------|-----------|----------|---------------|----------|-------------|
| Free       | 10        | 1 MB     | 1,000         | 2        | $0.00       |
| Basic      | 50        | 10 MB    | 10,000        | 5        | $9.99       |
| Pro        | 200       | 100 MB   | 100,000       | 20       | $29.99      |
| Enterprise | Unlimited | Unlimited| Unlimited     | Unlimited| $99.99      |

## Dependencies

### Core Dependencies
- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database toolkit with migrations
- `redis` - Redis client
- `serde` & `serde_json` - Serialization

### Authentication & Security
- `jsonwebtoken` - JWT authentication
- `argon2` - Password hashing
- `uuid` - Unique identifiers

### Utilities
- `chrono` - Date and time
- `dotenvy` - Environment variables
- `config` - Configuration management
- `tracing` & `tracing-subscriber` - Logging
- `anyhow` & `thiserror` - Error handling
- `validator` - Input validation
- `governor` - Rate limiting

### Terminal UI
- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `tokio-tungstenite` - WebSocket support

## Docker Services

### PostgreSQL
- Image: `postgres:16`
- Port: `5432`
- Database: `cloud_variables`
- User: `postgres`
- Password: `password` (change in production)

### Redis
- Image: `redis:7-alpine`
- Port: `6379`

### Managing Services

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f

# Restart services
docker-compose restart
```

## Development

### Build the Project

```bash
# Check for errors
cargo check

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## Troubleshooting

### Database Connection Issues

1. Ensure PostgreSQL container is running: `docker-compose ps`
2. Check database URL in `.env` file
3. Verify PostgreSQL is accepting connections: `docker-compose exec postgres pg_isready`

### Migration Failures

1. Check migration syntax in `migrations/` directory
2. Verify database connection
3. Check migration history: `sqlx migrate info`
4. Revert last migration if needed: `sqlx migrate revert`

### Redis Connection Issues

1. Ensure Redis container is running: `docker-compose ps`
2. Test Redis connection: `docker-compose exec redis redis-cli ping`
3. Check REDIS_URL in `.env` file

## Next Steps

After setting up the migration infrastructure, you can:

1. Implement API endpoints in `src/api/`
2. Create business logic services in `src/services/`
3. Add repository layer in `src/repositories/`
4. Implement middleware for authentication and rate limiting
5. Build the terminal UI in `src/terminal/`

## License

[Your License Here]

## Contributing

[Contributing Guidelines Here]
