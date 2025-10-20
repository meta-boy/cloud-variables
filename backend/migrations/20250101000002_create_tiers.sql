-- Create tiers table
CREATE TABLE IF NOT EXISTS tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    max_variables INTEGER NOT NULL,
    max_variable_size_mb INTEGER NOT NULL,
    max_requests_per_day INTEGER NOT NULL,
    max_api_keys INTEGER NOT NULL,
    price_monthly INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on name for faster lookups
CREATE INDEX idx_tiers_name ON tiers(name);

-- Create index on is_active for filtering
CREATE INDEX idx_tiers_is_active ON tiers(is_active);

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_tiers_updated_at
    BEFORE UPDATE ON tiers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraint to users table
ALTER TABLE users
ADD CONSTRAINT fk_users_tier_id
FOREIGN KEY (tier_id) REFERENCES tiers(id)
ON DELETE SET NULL;
