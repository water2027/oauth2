use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::shared::error::RepositoryError;

/// 数据库配置信息
pub struct PostgresConfig {
    pub url: String,
}

impl PostgresConfig {
    /// 从环境变量加载默认配置
    pub fn from_env() -> Result<Self, RepositoryError> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|e| RepositoryError::ConnectionError(format!("未设置 DATABASE_URL: {}", e)))?;
        Ok(Self { url })
    }
}

pub async fn setup_postgres_pool(config: &PostgresConfig) -> Result<PgPool, RepositoryError> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&config.url)
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))
}
