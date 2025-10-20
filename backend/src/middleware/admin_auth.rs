use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::error::{AppError, Result};
use crate::models::UserRole;
use crate::utils::Claims;

/// Middleware to ensure the user has admin role
pub async fn admin_middleware(req: Request, next: Next) -> Result<Response> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Authorization("User not authenticated".to_string()))?;

    if claims.role != UserRole::Admin {
        return Err(AppError::Authorization(
            "Admin access required".to_string(),
        ));
    }

    Ok(next.run(req).await)
}
