#!/bin/bash

# Cloud Variables - Run Database Migrations Script

set -e

echo "🔄 Running database migrations..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "⚠️  .env file not found. Using default DATABASE_URL."
    export DATABASE_URL="postgresql://postgres:password@localhost:5432/cloud_variables"
fi

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "❌ sqlx-cli is not installed."
    echo "   Install it with: cargo install sqlx-cli --no-default-features --features postgres"
    exit 1
fi

# Run migrations using sqlx-cli
echo "📊 Running migrations with sqlx-cli..."
sqlx migrate run --database-url "$DATABASE_URL"

echo "✅ Migrations completed successfully!"
echo ""
echo "You can also run migrations using the application:"
echo "  cargo run"
echo ""
