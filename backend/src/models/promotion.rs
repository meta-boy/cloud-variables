use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PromotionHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_tier_id: Uuid,
    pub to_tier_id: Uuid,
    pub promoted_by: Uuid, // Admin user ID
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
