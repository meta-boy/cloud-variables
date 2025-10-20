use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UsageStats {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub requests_count: i32,
    pub variables_created: i32,
    pub variables_updated: i32,
    pub variables_deleted: i32,
    pub variables_read: i32,
    pub total_bytes_stored: i64,
    pub total_bytes_transferred: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub user_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_requests: i32,
    pub total_variables_operations: i32,
    pub total_bytes_stored: i64,
    pub total_bytes_transferred: i64,
    pub current_variables_count: i32,
    pub current_api_keys_count: i32,
}
