use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tier {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_variables: i32,
    pub max_variable_size_mb: i32,
    pub max_requests_per_day: i32,
    pub max_api_keys: i32,
    pub price_monthly: i32, // in cents
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tier {
    pub fn can_create_variable(&self, current_count: i32) -> bool {
        current_count < self.max_variables
    }

    pub fn can_create_api_key(&self, current_count: i32) -> bool {
        current_count < self.max_api_keys
    }

    pub fn is_within_size_limit(&self, size_mb: i32) -> bool {
        size_mb <= self.max_variable_size_mb
    }

    pub fn is_within_rate_limit(&self, requests_today: i32) -> bool {
        requests_today < self.max_requests_per_day
    }
}
