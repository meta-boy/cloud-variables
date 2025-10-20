use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::Result;
use crate::models::Variable;

pub struct VariableRepository {
    pool: Pool<Postgres>,
}

impl VariableRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        key: &str,
        description: Option<&str>,
        size_bytes: i64,
        storage_path: &str,
        is_encrypted: bool,
        tags: Option<serde_json::Value>,
    ) -> Result<Variable> {
        let variable = sqlx::query_as::<_, Variable>(
            r#"
            INSERT INTO variables (user_id, key, description, size_bytes, storage_path,
                                 is_encrypted, tags, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 1)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(key)
        .bind(description)
        .bind(size_bytes)
        .bind(storage_path)
        .bind(is_encrypted)
        .bind(tags)
        .fetch_one(&self.pool)
        .await?;

        Ok(variable)
    }

    pub async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<Variable>> {
        let variable = sqlx::query_as::<_, Variable>(
            r#"
            SELECT * FROM variables WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(variable)
    }

    pub async fn find_by_key(&self, key: &str, user_id: Uuid) -> Result<Option<Variable>> {
        let variable = sqlx::query_as::<_, Variable>(
            r#"
            SELECT * FROM variables WHERE key = $1 AND user_id = $2
            "#,
        )
        .bind(key)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(variable)
    }

    pub async fn list(
        &self,
        user_id: Uuid,
        page: i32,
        page_size: i32,
        search: Option<&str>,
    ) -> Result<(Vec<Variable>, i64)> {
        let offset = (page - 1) * page_size;

        let mut query = String::from(
            r#"
            SELECT * FROM variables
            WHERE user_id = $1
            "#,
        );

        if search.is_some() {
            query.push_str(" AND key ILIKE '%' || $4 || '%'");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $2 OFFSET $3");

        let mut query_builder = sqlx::query_as::<_, Variable>(&query)
            .bind(user_id)
            .bind(page_size)
            .bind(offset);

        if let Some(s) = search {
            query_builder = query_builder.bind(s);
        }

        let variables = query_builder.fetch_all(&self.pool).await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM variables WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((variables, total.0))
    }

    pub async fn count_by_user(&self, user_id: Uuid) -> Result<i32> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM variables WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 as i32)
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        description: Option<&str>,
        size_bytes: Option<i64>,
        tags: Option<serde_json::Value>,
    ) -> Result<Variable> {
        let variable = sqlx::query_as::<_, Variable>(
            r#"
            UPDATE variables
            SET description = COALESCE($3, description),
                size_bytes = COALESCE($4, size_bytes),
                tags = COALESCE($5, tags),
                version = version + 1,
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(description)
        .bind(size_bytes)
        .bind(tags)
        .fetch_one(&self.pool)
        .await?;

        Ok(variable)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<Variable> {
        let variable = sqlx::query_as::<_, Variable>(
            r#"
            DELETE FROM variables WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(variable)
    }
}
