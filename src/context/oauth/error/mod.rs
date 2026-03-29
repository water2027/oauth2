use thiserror::Error;
use crate::shared::error::RepositoryError;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("不支持的挑战码方法, 仅支持SHA256")]
    UnsupportedCodeChallengeMethod,
    #[error("重定向地址数量错误")]
    InvalidRedirectUriCount,
    #[error("重定向地址不能超过5个")]
    TooManyRedirectUri,
    #[error("重定向地址必须使用https或者localhost")]
    UnsafeRedirectUri,
    #[error("无效的客户端")]
    InvalidClient,
    #[error("无效的重定向地址")]
    InvalidRedirectUri,
    #[error("内部错误: {0}")]
    InternalError(String),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
