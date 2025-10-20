use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::UserRole;
use crate::utils::{Claims, JwtConfig};

/// Extract and validate JWT token from Authorization header
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Authentication("Invalid authorization format".to_string()))?;

    let jwt_config = JwtConfig::from_env();
    let claims = jwt_config.verify_token(token)?;

    // Store claims in request extensions for later use
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Extension trait to easily extract authenticated user info from request
pub trait AuthenticatedUser {
    fn user_id(&self) -> Result<Uuid>;
    fn user_email(&self) -> Result<String>;
    fn user_role(&self) -> Result<UserRole>;
    fn tier_id(&self) -> Result<Uuid>;
}

impl AuthenticatedUser for Request {
    fn user_id(&self) -> Result<Uuid> {
        self.extensions()
            .get::<Claims>()
            .ok_or_else(|| AppError::Authentication("User not authenticated".to_string()))?
            .user_id()
    }

    fn user_email(&self) -> Result<String> {
        Ok(self
            .extensions()
            .get::<Claims>()
            .ok_or_else(|| AppError::Authentication("User not authenticated".to_string()))?
            .email
            .clone())
    }

    fn user_role(&self) -> Result<UserRole> {
        Ok(self
            .extensions()
            .get::<Claims>()
            .ok_or_else(|| AppError::Authentication("User not authenticated".to_string()))?
            .role)
    }

    fn tier_id(&self) -> Result<Uuid> {
        self.extensions()
            .get::<Claims>()
            .ok_or_else(|| AppError::Authentication("User not authenticated".to_string()))?
            .tier_id()
    }
}
