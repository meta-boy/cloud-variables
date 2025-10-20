use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use crate::dto::{PromoteUserRequest, PromotionResponse};
use crate::error::{AppError, Result};
use crate::repositories::{PromotionRepository, UserRepository};
use crate::utils::Claims;

pub async fn promote_user(
    State(pool): State<Pool<Postgres>>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<PromoteUserRequest>,
) -> Result<(StatusCode, Json<PromotionResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let admin_id = claims.user_id()?;
    let tier_id = Uuid::parse_str(&payload.tier_id)
        .map_err(|_| AppError::Validation("Invalid tier ID".to_string()))?;

    let user_repo = UserRepository::new(pool.clone());
    let promo_repo = PromotionRepository::new(pool);

    // Get user
    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let from_tier_id = user.tier_id;

    // Update user tier
    let updated_user = user_repo.update_tier(user_id, tier_id).await?;

    // Create promotion history
    let promotion = promo_repo
        .create(
            user_id,
            from_tier_id,
            tier_id,
            admin_id,
            payload.reason.as_deref(),
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(PromotionResponse {
            promotion,
            user: updated_user.sanitize(),
        }),
    ))
}
