use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

/// Database connection pool configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/cloud_variables".to_string()),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            min_connections: 2,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

impl DbConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
}

/// Creates and returns a PostgreSQL connection pool
pub async fn create_pool(config: &DbConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&config.database_url)
        .await
}

/// Creates a pool using environment variables
pub async fn create_pool_from_env() -> Result<Pool<Postgres>, sqlx::Error> {
    let config = DbConfig::from_env();
    create_pool(&config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
    }

    #[test]
    fn test_db_config_from_env() {
        unsafe {
            std::env::set_var("DATABASE_MAX_CONNECTIONS", "20");
        }
        let config = DbConfig::from_env();
        assert_eq!(config.max_connections, 20);
        unsafe {
            std::env::remove_var("DATABASE_MAX_CONNECTIONS");
        }
    }
}
