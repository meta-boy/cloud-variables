#!/bin/bash

# Cloud Variables - Create Admin User Script

set -e

echo "👤 Creating admin user..."

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "⚠️  .env file not found."
    exit 1
fi

# Prompt for admin details if not in environment
if [ -z "$ADMIN_EMAIL" ]; then
    read -p "Enter admin email: " ADMIN_EMAIL
fi

if [ -z "$ADMIN_PASSWORD" ]; then
    read -sp "Enter admin password: " ADMIN_PASSWORD
    echo ""
fi

# TODO: This script will be implemented once we have the auth service ready
# For now, we'll just create a placeholder SQL that can be executed manually

echo "📝 Generating admin user creation SQL..."

cat > /tmp/create_admin.sql << EOF
-- Create admin user
-- NOTE: You'll need to hash the password using argon2 before running this
-- This is a placeholder - use the application's auth service to create admin users

-- Example (replace with actual hashed password):
-- INSERT INTO users (email, password_hash, role, tier_id)
-- VALUES (
--     '$ADMIN_EMAIL',
--     '<argon2_hashed_password>',
--     'admin',
--     '00000000-0000-0000-0000-000000000001'
-- );

SELECT 'Admin user creation SQL template generated at /tmp/create_admin.sql';
EOF

echo "⚠️  Manual admin user creation required:"
echo "   1. Hash the password using argon2"
echo "   2. Update /tmp/create_admin.sql with the hashed password"
echo "   3. Execute the SQL against your database"
echo ""
echo "   Alternatively, implement an admin user creation endpoint in your application."
echo ""
