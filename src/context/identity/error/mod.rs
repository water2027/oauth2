use thiserror::Error;
use strum::EnumProperty;
use crate::shared::error::RepositoryError;

#[derive(Error, Debug, PartialEq, EnumProperty)]
pub enum DomainError {
    #[strum(props(code = "4041"))]
    #[error("用户不存在")]
    UserNotFound,

    #[strum(props(code = "4004"))]
    #[error("邮箱已存在")]
    EmailAlreadyExists,

    #[strum(props(code = "4001"))]
    #[error("非法的邮箱格式")]
    InvalidEmailFormat,

    #[strum(props(code = "4002"))]
    #[error("弱密码")]
    WeakPassword,

    #[strum(props(code = "4003"))]
    #[error("密码不匹配")]
    PasswordMismatch,

    #[strum(props(code = "4006"))]
    #[error("验证码无效")]
    InvalidValidationCode,

    #[strum(props(code = "4005"))]
    #[error("身份验证失败")]
    InvalidCredentials,

    #[strum(props(code = "5000"))]
    #[error("内部错误: {0}")]
    InternalError(String),

    #[strum(props(code = "5001"))]
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

impl DomainError {
    pub fn code(&self) -> i32 {
        self.get_str("code")
            .and_then(|c| c.parse().ok())
            .unwrap_or(5000)
    }
}
