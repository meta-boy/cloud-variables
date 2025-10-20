use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::Result;
use crate::models::Tier;

pub struct TierRepository {
    pool: Pool<Postgres>,
}

impl TierRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Tier>> {
        let tier = sqlx::query_as::<_, Tier>(
            r#"
            SELECT * FROM tiers WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tier)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Tier>> {
        let tier = sqlx::query_as::<_, Tier>(
            r#"
            SELECT * FROM tiers WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tier)
    }

    pub async fn list_active(&self) -> Result<Vec<Tier>> {
        let tiers = sqlx::query_as::<_, Tier>(
            r#"
            SELECT * FROM tiers WHERE is_active = true ORDER BY price_monthly ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tiers)
    }

    pub async fn list_all(&self) -> Result<Vec<Tier>> {
        let tiers = sqlx::query_as::<_, Tier>(
            r#"
            SELECT * FROM tiers ORDER BY price_monthly ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tiers)
    }

    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        max_variables: i32,
        max_variable_size_mb: i32,
        max_requests_per_day: i32,
        max_api_keys: i32,
        price_monthly: i32,
    ) -> Result<Tier> {
        let tier = sqlx::query_as::<_, Tier>(
            r#"
            INSERT INTO tiers (name, description, max_variables, max_variable_size_mb,
                             max_requests_per_day, max_api_keys, price_monthly, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(description)
        .bind(max_variables)
        .bind(max_variable_size_mb)
        .bind(max_requests_per_day)
        .bind(max_api_keys)
        .bind(price_monthly)
        .fetch_one(&self.pool)
        .await?;

        Ok(tier)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        max_variables: Option<i32>,
        max_variable_size_mb: Option<i32>,
        max_requests_per_day: Option<i32>,
        max_api_keys: Option<i32>,
        price_monthly: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Tier> {
        let mut query = String::from("UPDATE tiers SET updated_at = NOW()");
        let mut param_count = 1;

        if name.is_some() {
            param_count += 1;
            query.push_str(&format!(", name = ${}", param_count));
        }
        if description.is_some() {
            param_count += 1;
            query.push_str(&format!(", description = ${}", param_count));
        }
        if max_variables.is_some() {
            param_count += 1;
            query.push_str(&format!(", max_variables = ${}", param_count));
        }
        if max_variable_size_mb.is_some() {
            param_count += 1;
            query.push_str(&format!(", max_variable_size_mb = ${}", param_count));
        }
        if max_requests_per_day.is_some() {
            param_count += 1;
            query.push_str(&format!(", max_requests_per_day = ${}", param_count));
        }
        if max_api_keys.is_some() {
            param_count += 1;
            query.push_str(&format!(", max_api_keys = ${}", param_count));
        }
        if price_monthly.is_some() {
            param_count += 1;
            query.push_str(&format!(", price_monthly = ${}", param_count));
        }
        if is_active.is_some() {
            param_count += 1;
            query.push_str(&format!(", is_active = ${}", param_count));
        }

        query.push_str(" WHERE id = $1 RETURNING *");

        let mut query_builder = sqlx::query_as::<_, Tier>(&query).bind(id);

        if let Some(n) = name {
            query_builder = query_builder.bind(n);
        }
        if let Some(d) = description {
            query_builder = query_builder.bind(d);
        }
        if let Some(mv) = max_variables {
            query_builder = query_builder.bind(mv);
        }
        if let Some(ms) = max_variable_size_mb {
            query_builder = query_builder.bind(ms);
        }
        if let Some(mr) = max_requests_per_day {
            query_builder = query_builder.bind(mr);
        }
        if let Some(ma) = max_api_keys {
            query_builder = query_builder.bind(ma);
        }
        if let Some(pm) = price_monthly {
            query_builder = query_builder.bind(pm);
        }
        if let Some(ia) = is_active {
            query_builder = query_builder.bind(ia);
        }

        let tier = query_builder.fetch_one(&self.pool).await?;

        Ok(tier)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM tiers WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
