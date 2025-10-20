-- Create variables table
CREATE TABLE IF NOT EXISTS variables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    data_type VARCHAR(50) NOT NULL,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    storage_path TEXT NOT NULL,
    metadata JSONB,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create unique constraint on user_id and name
CREATE UNIQUE INDEX idx_variables_user_name ON variables(user_id, name) WHERE is_active = true;

-- Create index on user_id for faster lookups
CREATE INDEX idx_variables_user_id ON variables(user_id);

-- Create index on is_active for filtering
CREATE INDEX idx_variables_is_active ON variables(is_active);

-- Create index on created_at for sorting
CREATE INDEX idx_variables_created_at ON variables(created_at DESC);

-- Create GIN index on metadata for JSON queries
CREATE INDEX idx_variables_metadata ON variables USING GIN (metadata);

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_variables_updated_at
    BEFORE UPDATE ON variables
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraint
ALTER TABLE variables
ADD CONSTRAINT fk_variables_user_id
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;
