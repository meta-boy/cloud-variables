use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::error::Result;
use crate::models::PromotionHistory;

pub struct PromotionRepository {
    pool: Pool<Postgres>,
}

impl PromotionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        from_tier_id: Uuid,
        to_tier_id: Uuid,
        promoted_by: Uuid,
        reason: Option<&str>,
    ) -> Result<PromotionHistory> {
        let promotion = sqlx::query_as::<_, PromotionHistory>(
            r#"
            INSERT INTO promotion_history (user_id, from_tier_id, to_tier_id, promoted_by, reason)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(from_tier_id)
        .bind(to_tier_id)
        .bind(promoted_by)
        .bind(reason)
        .fetch_one(&self.pool)
        .await?;

        Ok(promotion)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<PromotionHistory>> {
        let promotions = sqlx::query_as::<_, PromotionHistory>(
            r#"
            SELECT * FROM promotion_history WHERE user_id = $1 ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(promotions)
    }
}
