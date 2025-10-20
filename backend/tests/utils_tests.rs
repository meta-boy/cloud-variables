mod common;

#[cfg(test)]
mod jwt_tests {
    use cloud_variables::models::UserRole;
    use cloud_variables::utils::JwtConfig;
    use uuid::Uuid;

    #[test]
    fn test_generate_and_verify_token() {
        let config = JwtConfig::new("test-secret-key-for-testing".to_string(), 24);

        let user_id = Uuid::new_v4();
        let tier_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        // Generate token
        let token = config
            .generate_token(user_id, email.clone(), UserRole::User, tier_id)
            .expect("Failed to generate token");

        assert!(!token.is_empty());

        // Verify token
        let claims = config.verify_token(&token).expect("Failed to verify token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, UserRole::User);
        assert_eq!(claims.tier_id, tier_id.to_string());
    }

    #[test]
    fn test_verify_invalid_token() {
        let config = JwtConfig::new("test-secret-key".to_string(), 24);

        let result = config.verify_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_token_with_wrong_secret() {
        let config1 = JwtConfig::new("secret1".to_string(), 24);
        let config2 = JwtConfig::new("secret2".to_string(), 24);

        let token = config1
            .generate_token(
                Uuid::new_v4(),
                "test@example.com".to_string(),
                UserRole::User,
                Uuid::new_v4(),
            )
            .unwrap();

        let result = config2.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_user_id_parsing() {
        let config = JwtConfig::new("test-secret".to_string(), 24);

        let user_id = Uuid::new_v4();
        let token = config
            .generate_token(
                user_id,
                "test@example.com".to_string(),
                UserRole::Admin,
                Uuid::new_v4(),
            )
            .unwrap();

        let claims = config.verify_token(&token).unwrap();
        let parsed_id = claims.user_id().unwrap();

        assert_eq!(parsed_id, user_id);
    }
}

#[cfg(test)]
mod hash_tests {
    use cloud_variables::utils::{
        extract_key_prefix, generate_api_key, hash_password, verify_password,
    };

    #[test]
    fn test_password_hashing() {
        let password = "my_secure_password_123";
        let hash = hash_password(password).expect("Failed to hash password");

        assert_ne!(hash, password);
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_password_verification_success() {
        let password = "correct_password";
        let hash = hash_password(password).unwrap();

        let result = verify_password(password, &hash).expect("Verification failed");
        assert!(result);
    }

    #[test]
    fn test_password_verification_failure() {
        let password = "correct_password";
        let hash = hash_password(password).unwrap();

        let result = verify_password("wrong_password", &hash).expect("Verification failed");
        assert!(!result);
    }

    #[test]
    fn test_generate_api_key_format() {
        let key = generate_api_key();

        assert!(key.starts_with("cv_"));
        assert_eq!(key.len(), 35); // cv_ + 32 chars
    }

    #[test]
    fn test_generate_api_key_uniqueness() {
        let key1 = generate_api_key();
        let key2 = generate_api_key();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_extract_key_prefix() {
        let key = "cv_abcdefgh123456789012345";
        let prefix = extract_key_prefix(&key);

        assert_eq!(prefix, "cv_abcdefgh");
        assert_eq!(prefix.len(), 11);
    }

    #[test]
    fn test_hash_password_different_for_same_input() {
        let password = "same_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        // Hashes should be different due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}

#[cfg(test)]
mod validation_tests {
    use cloud_variables::utils::{validate_api_key_name, validate_password, validate_variable_key};

    #[test]
    fn test_validate_password_success() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("MyP@ssw0rd").is_ok());
        assert!(validate_password("12345678a").is_ok());
    }

    #[test]
    fn test_validate_password_too_short() {
        assert!(validate_password("pass1").is_err());
        assert!(validate_password("1234567").is_err());
    }

    #[test]
    fn test_validate_password_no_number() {
        assert!(validate_password("passwordonly").is_err());
    }

    #[test]
    fn test_validate_password_no_letter() {
        assert!(validate_password("12345678").is_err());
    }

    #[test]
    fn test_validate_variable_key_success() {
        assert!(validate_variable_key("my_variable").is_ok());
        assert!(validate_variable_key("var-name-123").is_ok());
        assert!(validate_variable_key("config.json").is_ok());
        assert!(validate_variable_key("a").is_ok());
    }

    #[test]
    fn test_validate_variable_key_empty() {
        assert!(validate_variable_key("").is_err());
    }

    #[test]
    fn test_validate_variable_key_too_long() {
        let long_key = "a".repeat(256);
        assert!(validate_variable_key(&long_key).is_err());
    }

    #[test]
    fn test_validate_variable_key_invalid_chars() {
        assert!(validate_variable_key("invalid key").is_err());
        assert!(validate_variable_key("key@name").is_err());
        assert!(validate_variable_key("key#name").is_err());
    }

    #[test]
    fn test_validate_api_key_name_success() {
        assert!(validate_api_key_name("My API Key").is_ok());
        assert!(validate_api_key_name("Production Key").is_ok());
        assert!(validate_api_key_name("a").is_ok());
    }

    #[test]
    fn test_validate_api_key_name_empty() {
        assert!(validate_api_key_name("").is_err());
    }

    #[test]
    fn test_validate_api_key_name_too_long() {
        let long_name = "a".repeat(101);
        assert!(validate_api_key_name(&long_name).is_err());
    }
}

#[cfg(test)]
mod json_validator_tests {
    use cloud_variables::utils::{calculate_json_size, validate_json_data};
    use serde_json::json;

    #[test]
    fn test_calculate_json_size() {
        let data = json!({"key": "value"});
        let size = calculate_json_size(&data);

        assert!(size > 0);
        assert!(size < 100); // Small JSON
    }

    #[test]
    fn test_validate_json_data_within_limit() {
        let data = json!({
            "name": "test",
            "value": 123,
            "nested": {
                "field": "data"
            }
        });

        assert!(validate_json_data(&data, 10).is_ok());
    }

    #[test]
    fn test_validate_json_data_exceeds_limit() {
        // Create a large JSON (approximately 2MB)
        let large_string = "x".repeat(2 * 1024 * 1024);
        let data = json!({"data": large_string});

        let result = validate_json_data(&data, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_json_size_empty() {
        let data = json!({});
        let size = calculate_json_size(&data);

        assert_eq!(size, 2); // "{}"
    }

    #[test]
    fn test_calculate_json_size_array() {
        let data = json!([1, 2, 3, 4, 5]);
        let size = calculate_json_size(&data);

        assert!(size > 10);
    }
}
