mod common;

#[cfg(test)]
mod file_storage_tests {
    use cloud_variables::storage::{FileStorage, VariableStore};
    use serde_json::json;
    use tempfile::TempDir;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_file_storage_store_and_retrieve() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "test_variable";
        let data = json!({
            "name": "test",
            "value": 123,
            "nested": {
                "field": "data"
            }
        });

        // Store
        let path = storage
            .store(user_id, variable_key, &data)
            .await
            .expect("Failed to store");

        assert!(path.contains(&user_id.to_string()));
        assert!(path.contains(variable_key));

        // Retrieve
        let retrieved = storage.retrieve(&path).await.expect("Failed to retrieve");
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_file_storage_update() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "test_var";
        let initial_data = json!({"version": 1});
        let updated_data = json!({"version": 2, "new_field": "value"});

        // Store initial
        let path = storage.store(user_id, variable_key, &initial_data).await.unwrap();

        // Update
        storage.update(&path, &updated_data).await.expect("Failed to update");

        // Retrieve and verify
        let retrieved = storage.retrieve(&path).await.unwrap();
        assert_eq!(retrieved, updated_data);
        assert_ne!(retrieved, initial_data);
    }

    #[tokio::test]
    async fn test_file_storage_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "delete_test";
        let data = json!({"test": true});

        // Store
        let path = storage.store(user_id, variable_key, &data).await.unwrap();
        assert!(storage.exists(&path).await.unwrap());

        // Delete
        storage.delete(&path).await.expect("Failed to delete");

        // Verify deleted
        assert!(!storage.exists(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_file_storage_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "exists_test";
        let data = json!({"exists": true});

        // Should not exist initially
        let path = format!("{}/{}.json", user_id, variable_key);
        assert!(!storage.exists(&path).await.unwrap());

        // Store and verify exists
        let stored_path = storage.store(user_id, variable_key, &data).await.unwrap();
        assert!(storage.exists(&stored_path).await.unwrap());
    }

    #[tokio::test]
    async fn test_file_storage_retrieve_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let result = storage.retrieve("nonexistent/path.json").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_storage_update_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let data = json!({"test": true});
        let result = storage.update("nonexistent/path.json", &data).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_storage_multiple_users() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let variable_key = "same_key";

        let data1 = json!({"user": "user1"});
        let data2 = json!({"user": "user2"});

        // Store for both users with same key
        let path1 = storage.store(user1, variable_key, &data1).await.unwrap();
        let path2 = storage.store(user2, variable_key, &data2).await.unwrap();

        // Paths should be different
        assert_ne!(path1, path2);

        // Both should exist and have correct data
        let retrieved1 = storage.retrieve(&path1).await.unwrap();
        let retrieved2 = storage.retrieve(&path2).await.unwrap();

        assert_eq!(retrieved1, data1);
        assert_eq!(retrieved2, data2);
    }

    #[tokio::test]
    async fn test_file_storage_json_types() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();

        // Test various JSON types
        let test_cases = vec![
            ("string", json!("simple string")),
            ("number", json!(42)),
            ("boolean", json!(true)),
            ("null", json!(null)),
            ("array", json!([1, 2, 3, "four", true])),
            (
                "object",
                json!({
                    "nested": {
                        "deep": {
                            "value": "test"
                        }
                    }
                }),
            ),
        ];

        for (key, data) in test_cases {
            let path = storage.store(user_id, key, &data).await.unwrap();
            let retrieved = storage.retrieve(&path).await.unwrap();
            assert_eq!(retrieved, data, "Failed for type: {}", key);
        }
    }

    #[tokio::test]
    async fn test_file_storage_large_json() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "large_data";

        // Create a moderately large JSON
        let mut large_object = serde_json::Map::new();
        for i in 0..1000 {
            large_object.insert(format!("key_{}", i), json!(format!("value_{}", i)));
        }
        let data = json!(large_object);

        // Store and retrieve
        let path = storage.store(user_id, variable_key, &data).await.unwrap();
        let retrieved = storage.retrieve(&path).await.unwrap();

        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_file_storage_delete_idempotent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "delete_twice";
        let data = json!({"test": true});

        let path = storage.store(user_id, variable_key, &data).await.unwrap();

        // Delete twice should not error
        storage.delete(&path).await.expect("First delete failed");
        storage.delete(&path).await.expect("Second delete failed");

        assert!(!storage.exists(&path).await.unwrap());
    }
}
