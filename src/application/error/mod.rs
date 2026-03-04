use thiserror::Error;

use crate::domain::error::{DomainError, RepositoryError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("数据库错误: {0}")]
    Database(#[from] RepositoryError),
    #[error("资源不存在")]
    NotFound,
    #[error("权限不足")]
    Unauthorized,
    #[error("凭证无效")]
    InvalidCredentials,
}