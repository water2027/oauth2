use crate::shared::error::{RepositoryError};
use async_trait::async_trait;
use super::super::value_object::{email::Email, validation_code::ValidationCode};
use super::super::entity::validation_code::ValidationCodeRecord;
use super::super::error::DomainError;

#[async_trait]
pub trait ICodeSender: Send + Sync {
    async fn send(&self, code: &ValidationCodeRecord) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ICodeRepository: Send + Sync {
    fn generate_code(&self) -> ValidationCode;
    async fn save(&self, record: &ValidationCodeRecord) -> Result<(), RepositoryError>;
    async fn find(&self, email: &Email) -> Result<Option<ValidationCodeRecord>, RepositoryError>;
    async fn delete(&self, email: &Email) -> Result<(), RepositoryError>;
}
