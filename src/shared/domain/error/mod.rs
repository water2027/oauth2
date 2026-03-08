use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("非法的邮箱格式")]
    InvalidEmailFormat,
    #[error("弱密码")]
    WeakPassword,
    #[error("验证码无效")]
    InvalidValidationCode,
    #[error("内部错误: {0}")]
    InternalError(String),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryError {
    #[error("未找到")]
    NotFound,
    #[error("内部错误: {0}")]
    InternalError(String),
}