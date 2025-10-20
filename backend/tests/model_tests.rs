mod common;

#[cfg(test)]
mod role_tests {
    use cloud_variables::models::UserRole;

    #[test]
    fn test_user_role_is_admin() {
        assert!(UserRole::Admin.is_admin());
        assert!(!UserRole::User.is_admin());
    }

    #[test]
    fn test_user_role_default() {
        assert_eq!(UserRole::default(), UserRole::User);
    }

    #[test]
    fn test_user_role_display() {
        assert_eq!(UserRole::Admin.to_string(), "admin");
        assert_eq!(UserRole::User.to_string(), "user");
    }

    #[test]
    fn test_user_role_equality() {
        assert_eq!(UserRole::Admin, UserRole::Admin);
        assert_eq!(UserRole::User, UserRole::User);
        assert_ne!(UserRole::Admin, UserRole::User);
    }
}

#[cfg(test)]
mod tier_tests {
    use cloud_variables::models::Tier;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_tier() -> Tier {
        Tier {
            id: Uuid::new_v4(),
            name: "Free".to_string(),
            description: Some("Free tier".to_string()),
            max_variables: 10,
            max_variable_size_mb: 1,
            max_requests_per_day: 100,
            max_api_keys: 2,
            price_monthly: 0,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_tier_can_create_variable() {
        let tier = create_test_tier();

        assert!(tier.can_create_variable(5)); // Under limit
        assert!(tier.can_create_variable(9)); // Just under limit
        assert!(!tier.can_create_variable(10)); // At limit
        assert!(!tier.can_create_variable(15)); // Over limit
    }

    #[test]
    fn test_tier_can_create_api_key() {
        let tier = create_test_tier();

        assert!(tier.can_create_api_key(0));
        assert!(tier.can_create_api_key(1));
        assert!(!tier.can_create_api_key(2)); // At limit
        assert!(!tier.can_create_api_key(3));
    }

    #[test]
    fn test_tier_is_within_size_limit() {
        let tier = create_test_tier();

        assert!(tier.is_within_size_limit(0));
        assert!(tier.is_within_size_limit(1)); // At limit
        assert!(!tier.is_within_size_limit(2)); // Over limit
    }

    #[test]
    fn test_tier_is_within_rate_limit() {
        let tier = create_test_tier();

        assert!(tier.is_within_rate_limit(50));
        assert!(tier.is_within_rate_limit(99)); // Just under limit
        assert!(!tier.is_within_rate_limit(100)); // At limit
        assert!(!tier.is_within_rate_limit(150)); // Over limit
    }

    #[test]
    fn test_tier_edge_cases() {
        let tier = Tier {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
            description: None,
            max_variables: 1,
            max_variable_size_mb: 1,
            max_requests_per_day: 1,
            max_api_keys: 1,
            price_monthly: 0,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // With limit of 1, only 0 should be allowed
        assert!(tier.can_create_variable(0));
        assert!(!tier.can_create_variable(1));
    }
}

#[cfg(test)]
mod variable_tests {
    use cloud_variables::models::Variable;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_variable(size_bytes: i64) -> Variable {
        Variable {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            key: "test_key".to_string(),
            description: Some("Test variable".to_string()),
            size_bytes,
            version: 1,
            storage_path: "user_id/test_key.json".to_string(),
            is_encrypted: false,
            tags: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_variable_size_in_mb() {
        let var_1kb = create_test_variable(1024);
        assert_eq!(var_1kb.size_in_mb(), 0);

        let var_1mb = create_test_variable(1024 * 1024);
        assert_eq!(var_1mb.size_in_mb(), 1);

        let var_5mb = create_test_variable(5 * 1024 * 1024);
        assert_eq!(var_5mb.size_in_mb(), 5);

        let var_0 = create_test_variable(0);
        assert_eq!(var_0.size_in_mb(), 0);
    }

    #[test]
    fn test_variable_size_in_mb_rounding() {
        // 1.5 MB should round down to 1
        let var = create_test_variable(1024 * 1024 + 512 * 1024);
        assert_eq!(var.size_in_mb(), 1);

        // Just under 2 MB
        let var = create_test_variable(2 * 1024 * 1024 - 1);
        assert_eq!(var.size_in_mb(), 1);
    }
}

#[cfg(test)]
mod api_key_tests {
    use cloud_variables::models::ApiKey;
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    fn create_test_api_key(expires_at: Option<chrono::DateTime<Utc>>) -> ApiKey {
        ApiKey {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Test Key".to_string(),
            key_hash: "hashed_key".to_string(),
            prefix: "cv_abcdefgh".to_string(),
            last_used_at: None,
            expires_at,
            is_active: true,
            permissions: None,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_api_key_is_expired_future() {
        let future = Utc::now() + Duration::days(30);
        let key = create_test_api_key(Some(future));

        assert!(!key.is_expired());
    }

    #[test]
    fn test_api_key_is_expired_past() {
        let past = Utc::now() - Duration::days(1);
        let key = create_test_api_key(Some(past));

        assert!(key.is_expired());
    }

    #[test]
    fn test_api_key_is_expired_none() {
        let key = create_test_api_key(None);

        assert!(!key.is_expired());
    }

    #[test]
    fn test_api_key_is_valid_active_not_expired() {
        let future = Utc::now() + Duration::days(30);
        let key = create_test_api_key(Some(future));

        assert!(key.is_valid());
    }

    #[test]
    fn test_api_key_is_valid_inactive() {
        let mut key = create_test_api_key(None);
        key.is_active = false;

        assert!(!key.is_valid());
    }

    #[test]
    fn test_api_key_is_valid_expired() {
        let past = Utc::now() - Duration::days(1);
        let key = create_test_api_key(Some(past));

        assert!(!key.is_valid());
    }

    #[test]
    fn test_api_key_is_valid_inactive_and_expired() {
        let past = Utc::now() - Duration::days(1);
        let mut key = create_test_api_key(Some(past));
        key.is_active = false;

        assert!(!key.is_valid());
    }
}

#[cfg(test)]
mod user_tests {
    use cloud_variables::models::{User, UserRole};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_user(role: UserRole) -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            role,
            tier_id: Uuid::new_v4(),
            is_active: true,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_user_is_admin() {
        let admin = create_test_user(UserRole::Admin);
        assert!(admin.is_admin());

        let user = create_test_user(UserRole::User);
        assert!(!user.is_admin());
    }

    #[test]
    fn test_user_sanitize() {
        let user = create_test_user(UserRole::User);
        let user_id = user.id;

        let public_user = user.sanitize();

        assert_eq!(public_user.id, user_id);
        assert_eq!(public_user.email, "test@example.com");
        // PublicUser doesn't have password_hash field
    }

    #[test]
    fn test_user_sanitize_preserves_all_fields() {
        let user = create_test_user(UserRole::Admin);

        let public_user = user.clone().sanitize();

        assert_eq!(public_user.id, user.id);
        assert_eq!(public_user.email, user.email);
        assert_eq!(public_user.role, user.role);
        assert_eq!(public_user.tier_id, user.tier_id);
        assert_eq!(public_user.is_active, user.is_active);
        assert_eq!(public_user.email_verified, user.email_verified);
    }
}
