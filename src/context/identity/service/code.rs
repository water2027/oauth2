use std::sync::Arc;

use crate::context::identity::repository::code::ICodeSender;

use super::super::{
    error::DomainError,
    entity::validation_code::ValidationCodeRecord,
    repository::code::ICodeRepository,
    value_object::{email::Email, validation_code::ValidationCode}
};


pub struct CodeService {
    code_repository: Arc<dyn ICodeRepository>,
    code_sender: Arc<dyn ICodeSender>
}

impl CodeService {
    pub fn new(code_repository: Arc<dyn ICodeRepository>, code_sender: Arc<dyn ICodeSender>) -> Self {
        Self { code_repository, code_sender }
    }

    pub async fn send_code(&self, email: &Email) -> Result<(), DomainError> {
        let code = self.code_repository.generate_code();
        let record = ValidationCodeRecord::new(email.clone(), code, 300); // 5分钟过期
        self.code_sender.send(&record).await?;
        self.code_repository
            .save(&record)
            .await?;
        Ok(())
    }

    pub async fn verify_code(&self, email: &Email, attempt: &ValidationCode) -> Result<bool, DomainError> {
        let record = self.code_repository.find(email).await?;

        if let Some(record) = record && record.is_valid(attempt) {
            // 校验成功，用完即删
            self.code_repository.delete(email).await?;
            return Ok(true);
        }

        Ok(false)
    }
}
