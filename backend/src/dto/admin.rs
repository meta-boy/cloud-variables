use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{PromotionHistory, PublicUser};

#[derive(Debug, Deserialize, Validate)]
pub struct PromoteUserRequest {
    #[validate(length(min = 1))]
    pub tier_id: String, // UUID string

    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PromotionResponse {
    pub promotion: PromotionHistory,
    pub user: PublicUser,
}

#[derive(Debug, Serialize)]
pub struct UserManagementResponse {
    pub users: Vec<PublicUser>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub role: Option<String>,
    pub tier_id: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PlatformAnalytics {
    pub total_users: i64,
    pub active_users: i64,
    pub total_variables: i64,
    pub total_requests_today: i64,
    pub total_storage_bytes: i64,
    pub users_by_tier: Vec<TierUserCount>,
}

#[derive(Debug, Serialize)]
pub struct TierUserCount {
    pub tier_id: Uuid,
    pub tier_name: String,
    pub user_count: i64,
}
