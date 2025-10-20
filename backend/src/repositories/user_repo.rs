use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{User, UserRole};

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        tier_id: Uuid,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, role, tier_id, is_active, email_verified)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(UserRole::User)
        .bind(tier_id)
        .bind(true)
        .bind(false)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return AppError::Conflict("Email already exists".to_string());
                }
            }
            AppError::Database(e)
        })?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_email(&self, id: Uuid, email: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(password_hash)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_tier(&self, id: Uuid, tier_id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET tier_id = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(tier_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_status(&self, id: Uuid, is_active: bool) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET is_active = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(is_active)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn list(
        &self,
        page: i32,
        page_size: i32,
        search: Option<&str>,
    ) -> Result<(Vec<User>, i64)> {
        let offset = (page - 1) * page_size;

        let mut query = String::from(
            r#"
            SELECT * FROM users
            WHERE 1=1
            "#,
        );

        if search.is_some() {
            query.push_str(" AND email ILIKE '%' || $3 || '%'");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $1 OFFSET $2");

        let mut query_builder = sqlx::query_as::<_, User>(&query)
            .bind(page_size)
            .bind(offset);

        if let Some(s) = search {
            query_builder = query_builder.bind(s);
        }

        let users = query_builder.fetch_all(&self.pool).await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok((users, total.0))
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
