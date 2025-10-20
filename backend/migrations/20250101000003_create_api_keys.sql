-- Create api_keys table
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    key_hash VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255),
    last_used_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

-- Create index on key_hash for authentication
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);

-- Create index on is_active for filtering
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_api_keys_updated_at
    BEFORE UPDATE ON api_keys
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraint
ALTER TABLE api_keys
ADD CONSTRAINT fk_api_keys_user_id
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;
