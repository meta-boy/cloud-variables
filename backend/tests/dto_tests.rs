mod common;

#[cfg(test)]
mod auth_dto_tests {
    use cloud_variables::dto::{LoginRequest, RegisterRequest};
    use validator::Validate;

    #[test]
    fn test_register_request_valid() {
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_register_request_invalid_email() {
        let request = RegisterRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_register_request_short_password() {
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_login_request_valid() {
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "anypassword".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_login_request_invalid_email() {
        let request = LoginRequest {
            email: "not-an-email".to_string(),
            password: "password".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod variable_dto_tests {
    use cloud_variables::dto::{CreateVariableRequest, UpdateVariableRequest, VariableQueryParams};
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_create_variable_request_valid() {
        let request = CreateVariableRequest {
            key: "my_variable".to_string(),
            description: Some("Test variable".to_string()),
            data: json!({"test": true}),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            is_encrypted: false,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_variable_request_empty_key() {
        let request = CreateVariableRequest {
            key: "".to_string(),
            description: None,
            data: json!({}),
            tags: None,
            is_encrypted: false,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_variable_request_key_too_long() {
        let request = CreateVariableRequest {
            key: "a".repeat(256),
            description: None,
            data: json!({}),
            tags: None,
            is_encrypted: false,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_update_variable_request_valid() {
        let request = UpdateVariableRequest {
            description: Some("Updated description".to_string()),
            data: Some(json!({"updated": true})),
            tags: Some(vec!["new_tag".to_string()]),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_variable_request_all_none() {
        let request = UpdateVariableRequest {
            description: None,
            data: None,
            tags: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_variable_query_params_defaults() {
        let params = VariableQueryParams {
            page: None,
            page_size: None,
            search: None,
            tags: None,
        };

        assert!(params.page.is_none());
        assert!(params.page_size.is_none());
    }

    #[test]
    fn test_variable_query_params_with_values() {
        let params = VariableQueryParams {
            page: Some(2),
            page_size: Some(50),
            search: Some("test".to_string()),
            tags: Some("tag1,tag2".to_string()),
        };

        assert_eq!(params.page, Some(2));
        assert_eq!(params.page_size, Some(50));
        assert_eq!(params.search, Some("test".to_string()));
    }
}

#[cfg(test)]
mod user_dto_tests {
    use cloud_variables::dto::{ChangePasswordRequest, CreateApiKeyRequest};
    use validator::Validate;

    #[test]
    fn test_change_password_request_valid() {
        let request = ChangePasswordRequest {
            current_password: "oldpassword123".to_string(),
            new_password: "newpassword456".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_change_password_request_short_new_password() {
        let request = ChangePasswordRequest {
            current_password: "oldpassword".to_string(),
            new_password: "short".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_api_key_request_valid() {
        let request = CreateApiKeyRequest {
            name: "Production Key".to_string(),
            expires_in_days: Some(30),
            permissions: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_api_key_request_empty_name() {
        let request = CreateApiKeyRequest {
            name: "".to_string(),
            expires_in_days: None,
            permissions: None,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_api_key_request_name_too_long() {
        let request = CreateApiKeyRequest {
            name: "a".repeat(101),
            expires_in_days: None,
            permissions: None,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_api_key_request_no_expiration() {
        let request = CreateApiKeyRequest {
            name: "Permanent Key".to_string(),
            expires_in_days: None,
            permissions: None,
        };

        assert!(request.validate().is_ok());
    }
}

#[cfg(test)]
mod tier_dto_tests {
    use cloud_variables::dto::{CreateTierRequest, UpdateTierRequest};
    use validator::Validate;

    #[test]
    fn test_create_tier_request_valid() {
        let request = CreateTierRequest {
            name: "Premium".to_string(),
            description: Some("Premium tier".to_string()),
            max_variables: 100,
            max_variable_size_mb: 10,
            max_requests_per_day: 10000,
            max_api_keys: 5,
            price_monthly: 999,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_tier_request_invalid_negative_values() {
        let request = CreateTierRequest {
            name: "Test".to_string(),
            description: None,
            max_variables: 0, // Should be at least 1
            max_variable_size_mb: 1,
            max_requests_per_day: 1,
            max_api_keys: 1,
            price_monthly: 0,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_update_tier_request_partial() {
        let request = UpdateTierRequest {
            name: Some("Updated Name".to_string()),
            description: None,
            max_variables: Some(200),
            max_variable_size_mb: None,
            max_requests_per_day: None,
            max_api_keys: None,
            price_monthly: Some(1999),
            is_active: Some(true),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_tier_request_all_none() {
        let request = UpdateTierRequest {
            name: None,
            description: None,
            max_variables: None,
            max_variable_size_mb: None,
            max_requests_per_day: None,
            max_api_keys: None,
            price_monthly: None,
            is_active: None,
        };

        assert!(request.validate().is_ok());
    }
}

#[cfg(test)]
mod admin_dto_tests {
    use cloud_variables::dto::{PromoteUserRequest, UpdateUserRequest, UserQueryParams};
    use uuid::Uuid;
    use validator::Validate;

    #[test]
    fn test_promote_user_request_valid() {
        let request = PromoteUserRequest {
            tier_id: Uuid::new_v4().to_string(),
            reason: Some("Good performance".to_string()),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_promote_user_request_no_reason() {
        let request = PromoteUserRequest {
            tier_id: Uuid::new_v4().to_string(),
            reason: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_promote_user_request_empty_tier_id() {
        let request = PromoteUserRequest {
            tier_id: "".to_string(),
            reason: None,
        };

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_update_user_request() {
        let request = UpdateUserRequest {
            is_active: Some(false),
            email_verified: Some(true),
        };

        // UpdateUserRequest doesn't have validation, so we just construct it
        assert!(request.is_active == Some(false));
    }

    #[test]
    fn test_user_query_params() {
        let params = UserQueryParams {
            page: Some(1),
            page_size: Some(20),
            search: Some("john".to_string()),
            role: Some("admin".to_string()),
            tier_id: Some(Uuid::new_v4().to_string()),
            is_active: Some(true),
        };

        assert_eq!(params.page, Some(1));
        assert_eq!(params.search, Some("john".to_string()));
    }
}
