use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::storage::VariableStore;

#[derive(Clone)]
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.base_path).await?;
        Ok(())
    }

    fn get_user_dir(&self, user_id: Uuid) -> PathBuf {
        self.base_path.join(user_id.to_string())
    }

    fn get_variable_path(&self, user_id: Uuid, variable_key: &str) -> PathBuf {
        self.get_user_dir(user_id).join(format!("{}.json", variable_key))
    }
}

#[async_trait]
impl VariableStore for FileStorage {
    async fn store(&self, user_id: Uuid, variable_key: &str, data: &Value) -> Result<String> {
        let user_dir = self.get_user_dir(user_id);
        fs::create_dir_all(&user_dir).await?;

        let file_path = self.get_variable_path(user_id, variable_key);
        let json_string = serde_json::to_string_pretty(data)?;

        fs::write(&file_path, json_string).await?;

        // Return relative path from base
        Ok(format!("{}/{}.json", user_id, variable_key))
    }

    async fn retrieve(&self, storage_path: &str) -> Result<Value> {
        let full_path = self.base_path.join(storage_path);

        if !full_path.exists() {
            return Err(AppError::NotFound("Variable data not found".to_string()));
        }

        let contents = fs::read_to_string(full_path).await?;
        let data: Value = serde_json::from_str(&contents)?;

        Ok(data)
    }

    async fn update(&self, storage_path: &str, data: &Value) -> Result<()> {
        let full_path = self.base_path.join(storage_path);

        if !full_path.exists() {
            return Err(AppError::NotFound("Variable data not found".to_string()));
        }

        let json_string = serde_json::to_string_pretty(data)?;
        fs::write(full_path, json_string).await?;

        Ok(())
    }

    async fn delete(&self, storage_path: &str) -> Result<()> {
        let full_path = self.base_path.join(storage_path);

        if full_path.exists() {
            fs::remove_file(full_path).await?;
        }

        Ok(())
    }

    async fn exists(&self, storage_path: &str) -> Result<bool> {
        let full_path = self.base_path.join(storage_path);
        Ok(full_path.exists())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStorage::new(temp_dir.path());
        storage.init().await.unwrap();

        let user_id = Uuid::new_v4();
        let variable_key = "test_var";
        let data = json!({"key": "value", "number": 42});

        // Store
        let path = storage.store(user_id, variable_key, &data).await.unwrap();
        assert!(storage.exists(&path).await.unwrap());

        // Retrieve
        let retrieved = storage.retrieve(&path).await.unwrap();
        assert_eq!(retrieved, data);

        // Update
        let new_data = json!({"key": "new_value"});
        storage.update(&path, &new_data).await.unwrap();
        let retrieved = storage.retrieve(&path).await.unwrap();
        assert_eq!(retrieved, new_data);

        // Delete
        storage.delete(&path).await.unwrap();
        assert!(!storage.exists(&path).await.unwrap());
    }
}
