use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Variable {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key: String,
    pub description: Option<String>,
    pub size_bytes: i64,
    pub version: i32,
    pub storage_path: String,
    pub is_encrypted: bool,
    pub tags: Option<sqlx::types::JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Variable {
    pub fn size_in_mb(&self) -> i32 {
        (self.size_bytes / (1024 * 1024)) as i32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableWithData {
    #[serde(flatten)]
    pub variable: Variable,
    pub data: serde_json::Value,
}
