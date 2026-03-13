use thiserror::Error;
use crate::shared::error::RepositoryError;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("重定向地址必须使用https或者localhost")]
    UnsafeRedirectUri,
    #[error("内部错误: {0}")]
    InternalError(String),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

