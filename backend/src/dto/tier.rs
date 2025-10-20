use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Tier;

#[derive(Debug, Serialize)]
pub struct TierResponse {
    pub tier: Tier,
}

#[derive(Debug, Serialize)]
pub struct TierListResponse {
    pub tiers: Vec<Tier>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTierRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub description: Option<String>,

    #[validate(range(min = 1))]
    pub max_variables: i32,

    #[validate(range(min = 1))]
    pub max_variable_size_mb: i32,

    #[validate(range(min = 1))]
    pub max_requests_per_day: i32,

    #[validate(range(min = 1))]
    pub max_api_keys: i32,

    pub price_monthly: i32, // in cents
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTierRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_variables: Option<i32>,
    pub max_variable_size_mb: Option<i32>,
    pub max_requests_per_day: Option<i32>,
    pub max_api_keys: Option<i32>,
    pub price_monthly: Option<i32>,
    pub is_active: Option<bool>,
}
