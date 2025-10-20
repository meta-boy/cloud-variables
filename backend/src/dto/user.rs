use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{ApiKey, PublicUser};

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub user: PublicUser,
    pub tier_name: String,
    pub variables_count: i32,
    pub api_keys_count: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    pub expires_in_days: Option<i32>,

    pub permissions: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub api_key: ApiKey,
    pub secret: Option<String>, // Only populated on creation
}

#[derive(Debug, Serialize)]
pub struct ApiKeyListResponse {
    pub api_keys: Vec<ApiKey>,
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct UsageStatsResponse {
    pub user_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_requests: i32,
    pub variables_created: i32,
    pub variables_updated: i32,
    pub variables_deleted: i32,
    pub variables_read: i32,
    pub total_bytes_stored: i64,
    pub total_bytes_transferred: i64,
    pub tier_limits: TierLimits,
}

#[derive(Debug, Serialize)]
pub struct TierLimits {
    pub max_variables: i32,
    pub current_variables: i32,
    pub max_requests_per_day: i32,
    pub requests_today: i32,
    pub max_variable_size_mb: i32,
    pub max_api_keys: i32,
    pub current_api_keys: i32,
}
