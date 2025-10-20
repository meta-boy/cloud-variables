use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use crate::dto::{CreateTierRequest, TierListResponse, TierResponse, UpdateTierRequest};
use crate::error::{AppError, Result};
use crate::repositories::TierRepository;

pub async fn create_tier(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateTierRequest>,
) -> Result<(StatusCode, Json<TierResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let tier_repo = TierRepository::new(pool);

    let tier = tier_repo
        .create(
            &payload.name,
            payload.description.as_deref(),
            payload.max_variables,
            payload.max_variable_size_mb,
            payload.max_requests_per_day,
            payload.max_api_keys,
            payload.price_monthly,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(TierResponse { tier })))
}

pub async fn list_tiers(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<TierListResponse>> {
    let tier_repo = TierRepository::new(pool);
    let tiers = tier_repo.list_all().await?;

    Ok(Json(TierListResponse { tiers }))
}

pub async fn get_tier(
    State(pool): State<Pool<Postgres>>,
    Path(tier_id): Path<Uuid>,
) -> Result<Json<TierResponse>> {
    let tier_repo = TierRepository::new(pool);

    let tier = tier_repo
        .find_by_id(tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Tier not found".to_string()))?;

    Ok(Json(TierResponse { tier }))
}

pub async fn update_tier(
    State(pool): State<Pool<Postgres>>,
    Path(tier_id): Path<Uuid>,
    Json(payload): Json<UpdateTierRequest>,
) -> Result<Json<TierResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let tier_repo = TierRepository::new(pool);

    let tier = tier_repo
        .update(
            tier_id,
            payload.name.as_deref(),
            payload.description.as_deref(),
            payload.max_variables,
            payload.max_variable_size_mb,
            payload.max_requests_per_day,
            payload.max_api_keys,
            payload.price_monthly,
            payload.is_active,
        )
        .await?;

    Ok(Json(TierResponse { tier }))
}

pub async fn delete_tier(
    State(pool): State<Pool<Postgres>>,
    Path(tier_id): Path<Uuid>,
) -> Result<StatusCode> {
    let tier_repo = TierRepository::new(pool);
    tier_repo.delete(tier_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
