use thiserror::Error;
use crate::shared::error::RepositoryError;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("用户不存在")]
    UserNotFound,
    #[error("邮箱已存在")]
    EmailAlreadyExists,
    #[error("非法的邮箱格式")]
    InvalidEmailFormat,
    #[error("弱密码")]
    WeakPassword,
    #[error("密码不匹配")]
    PasswordMismatch,
    #[error("验证码无效")]
    InvalidValidationCode,
    #[error("内部错误: {0}")]
    InternalError(String),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
