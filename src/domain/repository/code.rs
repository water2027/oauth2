use async_trait::async_trait;
use crate::domain::{
    error::{DomainError, RepositoryError}, 
    value_object::{email::Email, validation_code::ValidationCode},
    entity::validation_code::ValidationCodeRecord,
};

#[async_trait]
pub trait ICodeSender: Send + Sync {
    async fn send(&self, email: &Email, code: &ValidationCode) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ICodeRepository: Send + Sync {
    fn generate_code(&self) -> ValidationCode;
    async fn save(&self, record: &ValidationCodeRecord) -> Result<(), RepositoryError>;
    async fn find(&self, email: &Email) -> Result<Option<ValidationCodeRecord>, RepositoryError>;
    async fn delete(&self, email: &Email) -> Result<(), RepositoryError>;
}