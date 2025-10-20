use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use crate::dto::{CreateVariableRequest, UpdateVariableRequest, VariableListResponse, VariableQueryParams, VariableResponse};
use crate::error::{AppError, Result};
use crate::repositories::{TierRepository, VariableRepository};
use crate::storage::{FileStorage, VariableStore};
use crate::utils::{validate_json_data, validate_variable_key, Claims};

pub async fn create_variable(
    State(pool): State<Pool<Postgres>>,
    State(storage): State<FileStorage>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateVariableRequest>,
) -> Result<(StatusCode, Json<VariableResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    validate_variable_key(&payload.key).map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = claims.user_id()?;
    let tier_id = claims.tier_id()?;

    let var_repo = VariableRepository::new(pool.clone());
    let tier_repo = TierRepository::new(pool);

    // Get user's tier limits
    let tier = tier_repo
        .find_by_id(tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

    // Check variable count limit
    let var_count = var_repo.count_by_user(user_id).await?;
    if !tier.can_create_variable(var_count) {
        return Err(AppError::TierLimitExceeded(format!(
            "Maximum {} variables allowed",
            tier.max_variables
        )));
    }

    // Check if variable key already exists
    if var_repo.find_by_key(&payload.key, user_id).await?.is_some() {
        return Err(AppError::Conflict("Variable key already exists".to_string()));
    }

    // Validate data size
    validate_json_data(&payload.data, tier.max_variable_size_mb)?;

    // Store variable data
    let storage_path = storage.store(user_id, &payload.key, &payload.data).await?;
    let size_bytes = crate::utils::json_validator::calculate_json_size(&payload.data) as i64;

    // Convert tags to JSON
    let tags_json = payload.tags.map(|tags| serde_json::json!(tags));

    // Create variable metadata in database
    let variable = var_repo
        .create(
            user_id,
            &payload.key,
            payload.description.as_deref(),
            size_bytes,
            &storage_path,
            payload.is_encrypted,
            tags_json,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(VariableResponse {
            variable,
            data: Some(payload.data),
        }),
    ))
}

pub async fn get_variable(
    State(pool): State<Pool<Postgres>>,
    State(storage): State<FileStorage>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<VariableResponse>> {
    let user_id = claims.user_id()?;
    let var_repo = VariableRepository::new(pool);

    let variable = var_repo
        .find_by_id(id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Variable not found".to_string()))?;

    // Retrieve data from storage
    let data = storage.retrieve(&variable.storage_path).await?;

    Ok(Json(VariableResponse {
        variable,
        data: Some(data),
    }))
}

pub async fn list_variables(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<VariableQueryParams>,
) -> Result<Json<VariableListResponse>> {
    let user_id = claims.user_id()?;
    let var_repo = VariableRepository::new(pool);

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).min(100).max(1);

    let (variables, total) = var_repo
        .list(user_id, page, page_size, params.search.as_deref())
        .await?;

    Ok(Json(VariableListResponse {
        variables,
        total,
        page,
        page_size,
    }))
}

pub async fn update_variable(
    State(pool): State<Pool<Postgres>>,
    State(storage): State<FileStorage>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateVariableRequest>,
) -> Result<Json<VariableResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = claims.user_id()?;
    let tier_id = claims.tier_id()?;

    let var_repo = VariableRepository::new(pool.clone());
    let tier_repo = TierRepository::new(pool);

    // Get existing variable
    let variable = var_repo
        .find_by_id(id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Variable not found".to_string()))?;

    // If updating data, validate size and update storage
    let new_size = if let Some(ref data) = payload.data {
        let tier = tier_repo
            .find_by_id(tier_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

        validate_json_data(data, tier.max_variable_size_mb)?;
        storage.update(&variable.storage_path, data).await?;

        Some(crate::utils::json_validator::calculate_json_size(data) as i64)
    } else {
        None
    };

    // Convert tags to JSON
    let tags_json = payload.tags.map(|tags| serde_json::json!(tags));

    // Update variable metadata
    let updated_variable = var_repo
        .update(
            id,
            user_id,
            payload.description.as_deref(),
            new_size,
            tags_json,
        )
        .await?;

    // Retrieve current data
    let data = storage.retrieve(&updated_variable.storage_path).await?;

    Ok(Json(VariableResponse {
        variable: updated_variable,
        data: Some(data),
    }))
}

pub async fn delete_variable(
    State(pool): State<Pool<Postgres>>,
    State(storage): State<FileStorage>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let user_id = claims.user_id()?;
    let var_repo = VariableRepository::new(pool);

    // Delete from database and get the storage path
    let variable = var_repo.delete(id, user_id).await?;

    // Delete from storage
    storage.delete(&variable.storage_path).await?;

    Ok(StatusCode::NO_CONTENT)
}
