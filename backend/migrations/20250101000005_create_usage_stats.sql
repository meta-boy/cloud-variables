-- Create usage_stats table
CREATE TABLE IF NOT EXISTS usage_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    date DATE NOT NULL,
    api_calls_count INTEGER NOT NULL DEFAULT 0,
    storage_bytes_used BIGINT NOT NULL DEFAULT 0,
    variables_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create unique constraint on user_id and date
CREATE UNIQUE INDEX idx_usage_stats_user_date ON usage_stats(user_id, date);

-- Create index on user_id for faster lookups
CREATE INDEX idx_usage_stats_user_id ON usage_stats(user_id);

-- Create index on date for time-based queries
CREATE INDEX idx_usage_stats_date ON usage_stats(date DESC);

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_usage_stats_updated_at
    BEFORE UPDATE ON usage_stats
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraint
ALTER TABLE usage_stats
ADD CONSTRAINT fk_usage_stats_user_id
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;
