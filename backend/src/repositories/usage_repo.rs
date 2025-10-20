use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::Result;
use crate::models::UsageStats;

pub struct UsageRepository {
    pool: Pool<Postgres>,
}

impl UsageRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create_today(&self, user_id: Uuid) -> Result<UsageStats> {
        let today = Utc::now().date_naive();

        let stats = sqlx::query_as::<_, UsageStats>(
            r#"
            INSERT INTO usage_stats (user_id, date, requests_count, variables_created,
                                   variables_updated, variables_deleted, variables_read,
                                   total_bytes_stored, total_bytes_transferred)
            VALUES ($1, $2, 0, 0, 0, 0, 0, 0, 0)
            ON CONFLICT (user_id, date) DO UPDATE SET user_id = EXCLUDED.user_id
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(today)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    pub async fn increment_requests(&self, user_id: Uuid) -> Result<()> {
        let today = Utc::now().date_naive();

        sqlx::query(
            r#"
            INSERT INTO usage_stats (user_id, date, requests_count, variables_created,
                                   variables_updated, variables_deleted, variables_read,
                                   total_bytes_stored, total_bytes_transferred)
            VALUES ($1, $2, 1, 0, 0, 0, 0, 0, 0)
            ON CONFLICT (user_id, date) DO UPDATE SET requests_count = usage_stats.requests_count + 1
            "#,
        )
        .bind(user_id)
        .bind(today)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_requests_today(&self, user_id: Uuid) -> Result<i32> {
        let today = Utc::now().date_naive();

        let count: Option<(i32,)> = sqlx::query_as(
            "SELECT requests_count FROM usage_stats WHERE user_id = $1 AND date = $2"
        )
        .bind(user_id)
        .bind(today)
        .fetch_optional(&self.pool)
        .await?;

        Ok(count.map(|c| c.0).unwrap_or(0))
    }

    pub async fn get_stats_range(
        &self,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<UsageStats>> {
        let stats = sqlx::query_as::<_, UsageStats>(
            r#"
            SELECT * FROM usage_stats
            WHERE user_id = $1 AND date >= $2 AND date <= $3
            ORDER BY date DESC
            "#,
        )
        .bind(user_id)
        .bind(start.date_naive())
        .bind(end.date_naive())
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }
}
