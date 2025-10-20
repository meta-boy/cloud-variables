use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::Result;
use crate::models::ApiKey;

pub struct ApiKeyRepository {
    pool: Pool<Postgres>,
}

impl ApiKeyRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        key_hash: &str,
        prefix: &str,
        expires_in_days: Option<i32>,
    ) -> Result<ApiKey> {
        let expires_at = expires_in_days.map(|days| Utc::now() + Duration::days(days as i64));

        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys (user_id, name, key_hash, prefix, expires_at, is_active)
            VALUES ($1, $2, $3, $4, $5, true)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(name)
        .bind(key_hash)
        .bind(prefix)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(api_key)
    }

    pub async fn find_by_prefix(&self, prefix: &str) -> Result<Option<ApiKey>> {
        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT * FROM api_keys WHERE prefix = $1
            "#,
        )
        .bind(prefix)
        .fetch_optional(&self.pool)
        .await?;

        Ok(api_key)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>> {
        let api_keys = sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT * FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(api_keys)
    }

    pub async fn count_by_user(&self, user_id: Uuid) -> Result<i32> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM api_keys WHERE user_id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 as i32)
    }

    pub async fn update_last_used(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE api_keys SET last_used_at = NOW() WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn revoke(&self, id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE api_keys SET is_active = false WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM api_keys WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
