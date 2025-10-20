mod common;

#[cfg(test)]
mod error_handling_tests {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use cloud_variables::error::AppError;

    #[test]
    fn test_authentication_error_status() {
        let error = AppError::Authentication("Invalid credentials".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_authorization_error_status() {
        let error = AppError::Authorization("Admin required".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_not_found_error_status() {
        let error = AppError::NotFound("User not found".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_validation_error_status() {
        let error = AppError::Validation("Invalid input".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_bad_request_error_status() {
        let error = AppError::BadRequest("Bad request".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_tier_limit_exceeded_error_status() {
        let error = AppError::TierLimitExceeded("Limit exceeded".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
    }

    #[test]
    fn test_rate_limit_exceeded_error_status() {
        let error = AppError::RateLimitExceeded;
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn test_conflict_error_status() {
        let error = AppError::Conflict("Email already exists".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_internal_server_error_status() {
        let error = AppError::InternalServer("Something went wrong".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_database_error_status() {
        let db_error = sqlx::Error::RowNotFound;
        let error = AppError::Database(db_error);
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_password_hash_error_status() {
        let error = AppError::PasswordHash;
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_display() {
        let error = AppError::Authentication("Test message".to_string());
        assert_eq!(error.to_string(), "Authentication failed: Test message");

        let error = AppError::NotFound("User".to_string());
        assert_eq!(error.to_string(), "Not found: User");

        let error = AppError::RateLimitExceeded;
        assert_eq!(error.to_string(), "Rate limit exceeded");
    }

    #[test]
    fn test_error_from_sqlx() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let app_error: AppError = sqlx_error.into();

        matches!(app_error, AppError::Database(_));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();

        matches!(app_error, AppError::Io(_));
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_str = "{invalid json}";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let app_error: AppError = json_error.into();

        matches!(app_error, AppError::Json(_));
    }
}
