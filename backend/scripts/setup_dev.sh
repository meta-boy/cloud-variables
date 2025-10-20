#!/bin/bash

# Cloud Variables - Development Environment Setup Script

set -e

echo "🚀 Setting up Cloud Variables development environment..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    if [ -f .env.example ]; then
        cp .env.example .env
        echo "✅ .env file created. Please update it with your configuration."
    else
        echo "⚠️  .env.example not found. Creating basic .env file..."
        cat > .env << EOF
# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/cloud_variables
DATABASE_MAX_CONNECTIONS=10

# Redis
REDIS_URL=redis://localhost:6379

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# JWT
JWT_SECRET=$(openssl rand -base64 32)
JWT_EXPIRATION_HOURS=24

# Storage
STORAGE_TYPE=filesystem
STORAGE_PATH=./data/variables

# Admin
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=change-this-password

# Environment
RUST_LOG=info,cloud_variables=debug
ENVIRONMENT=development
EOF
        echo "✅ Basic .env file created with random JWT secret."
    fi
else
    echo "✅ .env file already exists."
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "⚠️  Docker is not running. Please start Docker and run this script again."
    exit 1
fi

echo "🐳 Starting Docker containers..."
docker-compose up -d postgres redis

echo "⏳ Waiting for PostgreSQL to be ready..."
sleep 5

# Wait for PostgreSQL to be ready
until docker-compose exec -T postgres pg_isready -U postgres > /dev/null 2>&1; do
    echo "  Waiting for PostgreSQL..."
    sleep 2
done

echo "✅ PostgreSQL is ready!"

# Create data directory for file storage
echo "📁 Creating data directories..."
mkdir -p data/variables
echo "✅ Data directories created."

# Install sqlx-cli if not already installed
if ! command -v sqlx &> /dev/null; then
    echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
else
    echo "✅ sqlx-cli is already installed."
fi

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "Next steps:"
echo "  1. Review and update .env file with your configuration"
echo "  2. Run migrations: ./scripts/run_migrations.sh"
echo "  3. Start the application: cargo run"
echo ""
