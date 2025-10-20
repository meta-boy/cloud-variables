use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::error::Result;

#[async_trait]
pub trait VariableStore: Send + Sync {
    /// Store variable data and return the storage path
    async fn store(&self, user_id: Uuid, variable_key: &str, data: &Value) -> Result<String>;

    /// Retrieve variable data from storage
    async fn retrieve(&self, storage_path: &str) -> Result<Value>;

    /// Update variable data at the given storage path
    async fn update(&self, storage_path: &str, data: &Value) -> Result<()>;

    /// Delete variable data from storage
    async fn delete(&self, storage_path: &str) -> Result<()>;

    /// Check if data exists at the storage path
    async fn exists(&self, storage_path: &str) -> Result<bool>;
}
