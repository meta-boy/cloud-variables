use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use crate::dto::{
    ApiKeyListResponse, ApiKeyResponse, ChangePasswordRequest, CreateApiKeyRequest,
    UserProfileResponse,
};
use crate::error::{AppError, Result};
use crate::repositories::{ApiKeyRepository, TierRepository, UserRepository, VariableRepository};
use crate::utils::{extract_key_prefix, generate_api_key, hash_password, verify_password, Claims};

pub async fn get_profile(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserProfileResponse>> {
    let user_id = claims.user_id()?;

    let user_repo = UserRepository::new(pool.clone());
    let tier_repo = TierRepository::new(pool.clone());
    let var_repo = VariableRepository::new(pool.clone());
    let key_repo = ApiKeyRepository::new(pool);

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let tier = tier_repo
        .find_by_id(user.tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

    let variables_count = var_repo.count_by_user(user_id).await?;
    let api_keys_count = key_repo.count_by_user(user_id).await?;

    Ok(Json(UserProfileResponse {
        user: user.sanitize(),
        tier_name: tier.name,
        variables_count,
        api_keys_count,
    }))
}

pub async fn change_password(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = claims.user_id()?;
    let user_repo = UserRepository::new(pool);

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Verify current password
    if !verify_password(&payload.current_password, &user.password_hash)? {
        return Err(AppError::Authentication("Current password is incorrect".to_string()));
    }

    // Hash new password
    let new_hash = hash_password(&payload.new_password)?;

    // Update password
    user_repo.update_password(user_id, &new_hash).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_api_key(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiKeyResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = claims.user_id()?;
    let tier_id = claims.tier_id()?;

    let key_repo = ApiKeyRepository::new(pool.clone());
    let tier_repo = TierRepository::new(pool);

    // Check API key count limit
    let tier = tier_repo
        .find_by_id(tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

    let key_count = key_repo.count_by_user(user_id).await?;
    if !tier.can_create_api_key(key_count) {
        return Err(AppError::TierLimitExceeded(format!(
            "Maximum {} API keys allowed",
            tier.max_api_keys
        )));
    }

    // Generate API key
    let api_key_secret = generate_api_key();
    let prefix = extract_key_prefix(&api_key_secret);
    let key_hash = hash_password(&api_key_secret)?;

    // Create API key
    let api_key = key_repo
        .create(
            user_id,
            &payload.name,
            &key_hash,
            &prefix,
            payload.expires_in_days,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiKeyResponse {
            api_key,
            secret: Some(api_key_secret),
        }),
    ))
}

pub async fn list_api_keys(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiKeyListResponse>> {
    let user_id = claims.user_id()?;
    let key_repo = ApiKeyRepository::new(pool);

    let api_keys = key_repo.list_by_user(user_id).await?;

    Ok(Json(ApiKeyListResponse {
        total: api_keys.len() as i32,
        api_keys,
    }))
}

pub async fn revoke_api_key(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Path(key_id): Path<Uuid>,
) -> Result<StatusCode> {
    let user_id = claims.user_id()?;
    let key_repo = ApiKeyRepository::new(pool);

    key_repo.revoke(key_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_api_key(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Path(key_id): Path<Uuid>,
) -> Result<StatusCode> {
    let user_id = claims.user_id()?;
    let key_repo = ApiKeyRepository::new(pool);

    key_repo.delete(key_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
