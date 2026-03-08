use thiserror::Error;
use crate::shared::domain::error::DomainError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("无效的凭证")]
    InvalidCredentials,
    #[error(transparent)]
    Domain(#[from] DomainError),
}
