use cloud_variables::db::create_pool;
use sqlx::{Pool, Postgres};
use std::env;

/// Create a test database pool
pub async fn setup_test_db() -> Pool<Postgres> {
    dotenvy::dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/cloud_variables_test".to_string());

    let config = cloud_variables::DbConfig {
        database_url,
        max_connections: 5,
        min_connections: 1,
        connect_timeout: std::time::Duration::from_secs(5),
        idle_timeout: std::time::Duration::from_secs(60),
    };

    create_pool(&config).await.expect("Failed to create test pool")
}

/// Clean up test database
pub async fn cleanup_test_db(pool: &Pool<Postgres>) {
    sqlx::query("TRUNCATE users, tiers, variables, api_keys, usage_stats, promotion_history CASCADE")
        .execute(pool)
        .await
        .ok();
}
