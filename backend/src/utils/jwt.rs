use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,    // User email
    pub role: UserRole,   // User role
    pub tier_id: String,  // Tier ID
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, role: UserRole, tier_id: Uuid, expires_in_hours: i64) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(expires_in_hours);

        Self {
            sub: user_id.to_string(),
            email,
            role,
            tier_id: tier_id.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))
    }

    pub fn tier_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.tier_id)
            .map_err(|_| AppError::Authentication("Invalid tier ID in token".to_string()))
    }
}

pub struct JwtConfig {
    secret: String,
    expiration_hours: i64,
}

impl JwtConfig {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: String, role: UserRole, tier_id: Uuid) -> Result<String> {
        let claims = Claims::new(user_id, email, role, tier_id, self.expiration_hours);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Jwt(e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Jwt(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_token() {
        let config = JwtConfig {
            secret: "test-secret".to_string(),
            expiration_hours: 24,
        };

        let user_id = Uuid::new_v4();
        let tier_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let role = UserRole::User;

        let token = config.generate_token(user_id, email.clone(), role, tier_id).unwrap();
        let claims = config.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }
}
