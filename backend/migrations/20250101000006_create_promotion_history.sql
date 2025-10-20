-- Create promotion_history table
CREATE TABLE IF NOT EXISTS promotion_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    from_tier_id UUID,
    to_tier_id UUID NOT NULL,
    promoted_by_user_id UUID,
    reason TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_promotion_history_user_id ON promotion_history(user_id);

-- Create index on created_at for time-based queries
CREATE INDEX idx_promotion_history_created_at ON promotion_history(created_at DESC);

-- Create index on from_tier_id for analytics
CREATE INDEX idx_promotion_history_from_tier_id ON promotion_history(from_tier_id);

-- Create index on to_tier_id for analytics
CREATE INDEX idx_promotion_history_to_tier_id ON promotion_history(to_tier_id);

-- Add foreign key constraints
ALTER TABLE promotion_history
ADD CONSTRAINT fk_promotion_history_user_id
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;

ALTER TABLE promotion_history
ADD CONSTRAINT fk_promotion_history_from_tier_id
FOREIGN KEY (from_tier_id) REFERENCES tiers(id)
ON DELETE SET NULL;

ALTER TABLE promotion_history
ADD CONSTRAINT fk_promotion_history_to_tier_id
FOREIGN KEY (to_tier_id) REFERENCES tiers(id)
ON DELETE CASCADE;

ALTER TABLE promotion_history
ADD CONSTRAINT fk_promotion_history_promoted_by_user_id
FOREIGN KEY (promoted_by_user_id) REFERENCES users(id)
ON DELETE SET NULL;
