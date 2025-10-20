use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::error::{AppError, Result};
use crate::repositories::{TierRepository, UserRepository};
use crate::utils::{hash_password, verify_password, JwtConfig};

pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_repo = UserRepository::new(pool.clone());
    let tier_repo = TierRepository::new(pool.clone());

    // Check if user already exists
    if user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Get the free/default tier
    let tiers = tier_repo.list_active().await?;
    let default_tier = tiers
        .into_iter()
        .find(|t| t.price_monthly == 0)
        .ok_or_else(|| AppError::InternalServer("No default tier available".to_string()))?;

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = user_repo
        .create(&payload.email, &password_hash, default_tier.id)
        .await?;

    // Generate JWT token
    let jwt_config = JwtConfig::from_env();
    let token = jwt_config.generate_token(user.id, user.email.clone(), user.role, user.tier_id)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: user.sanitize(),
            token,
        }),
    ))
}

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_repo = UserRepository::new(pool);

    // Find user by email
    let user = user_repo
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Authentication("Account is inactive".to_string()));
    }

    // Generate JWT token
    let jwt_config = JwtConfig::from_env();
    let token = jwt_config.generate_token(user.id, user.email.clone(), user.role, user.tier_id)?;

    Ok(Json(AuthResponse {
        user: user.sanitize(),
        token,
    }))
}
