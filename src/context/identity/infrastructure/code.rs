use chrono::{Utc};
use redis::AsyncCommands;
use serde_json;
use std::sync::Arc;
use async_trait::async_trait;
use rand::{self, RngExt};

use super::super::value_object::{Email, ValidationCode};
use super::super::entity::ValidationCodeRecord;
use super::super::error::DomainError;
use super::super::repository::code::{ICodeRepository, ICodeSender, ICodeGenerator};
use crate::shared::infrastructure::email::EmailSender;
use crate::shared::error::RepositoryError;

pub struct EmailCodeSender {
    email_sender: Arc<dyn EmailSender>,
}

impl EmailCodeSender {
    pub fn new(email_sender: Arc<dyn EmailSender>) -> Self {
        Self { email_sender }
    }
}

#[async_trait]
impl ICodeSender for EmailCodeSender {
    async fn send(&self, code: &ValidationCodeRecord) -> Result<(), DomainError> {
        let to = code.email.as_ref();
        let subject = "验证码 - OAuth2 服务";
        let body = format!("您的验证码是：{}，有效期为 5 分钟。", code.code.as_ref());

        self.email_sender
            .send_email(to, subject, &body)
            .await
            .map_err(|e| DomainError::InternalError(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}

pub struct RedisCodeRepository {
    client: redis::Client,
}

impl RedisCodeRepository {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ICodeRepository for RedisCodeRepository {
    async fn save(&self, record: &ValidationCodeRecord) -> Result<(), RepositoryError> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::ConnectionError(format!("Redis async connection failed: {}", e)))?;

        let key = format!("validation_code:{}", record.email.as_ref());
        let value = serde_json::to_string(record)
            .map_err(|e| RepositoryError::InternalError(format!("Serialization failed: {}", e)))?;

        let now = Utc::now();
        let ttl_secs = (record.expires_at - now).num_seconds();
        let ttl_secs = if ttl_secs > 0 { ttl_secs as u64 } else { 0 };

        if ttl_secs > 0 {
            let _: () = conn.set_ex(&key, value, ttl_secs)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("Redis set_ex failed: {}", e)))?;
        }

        Ok(())
    }

    async fn find(&self, email: &Email) -> Result<Option<ValidationCodeRecord>, RepositoryError> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::ConnectionError(format!("Redis async connection failed: {}", e)))?;

        let key = format!("validation_code:{}", email.as_ref());
        let value: Option<String> = conn.get(&key)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("Redis get failed: {}", e)))?;

        if let Some(v) = value {
            let record: ValidationCodeRecord = serde_json::from_str(&v)
                .map_err(|e| RepositoryError::InternalError(format!("Deserialization failed: {}", e)))?;
            return Ok(Some(record));
        }

        Ok(None)
    }

    async fn delete(&self, email: &Email) -> Result<(), RepositoryError> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::ConnectionError(format!("Redis async connection failed: {}", e)))?;

        let key = format!("validation_code:{}", email.as_ref());
        let _: () = conn.del(&key)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("Redis del failed: {}", e)))?;

        Ok(())
    }
}

pub struct SimpleCodeGenerator;

impl ICodeGenerator for SimpleCodeGenerator {
    fn generate_code(&self) -> Result<ValidationCode, DomainError> {
        let mut rng = rand::rng();
        let code: u32 = rng.random_range(100000..=999999);
        ValidationCode::new(code.to_string())
    }
}
