use redis::{Client};
use crate::shared::error::RepositoryError;

pub struct RedisConfig {
    pub url: String
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, RepositoryError> {
        let url = std::env::var("REDIS_URL")
            .map_err(|e| RepositoryError::ConnectionError(format!("未设置 REDIS_URL: {}", e)))?;
        Ok(Self { url })
    }
}

pub fn setup_redis_client(config: &RedisConfig) -> Result<Client, RepositoryError> {
    Client::open(config.url.as_str()).map_err(|e| RepositoryError::ConnectionError(format!("Redis 连接失败: {}", e)))
}