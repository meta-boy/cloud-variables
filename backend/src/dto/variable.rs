use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::models::Variable;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVariableRequest {
    #[validate(length(min = 1, max = 255, message = "Key must be between 1 and 255 characters"))]
    pub key: String,

    pub description: Option<String>,

    pub data: Value,

    pub tags: Option<Vec<String>>,

    #[serde(default)]
    pub is_encrypted: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVariableRequest {
    pub description: Option<String>,

    pub data: Option<Value>,

    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct VariableResponse {
    #[serde(flatten)]
    pub variable: Variable,
    pub data: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct VariableListResponse {
    pub variables: Vec<Variable>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct VariableQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub tags: Option<String>, // Comma-separated tags
}
