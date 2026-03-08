use std::time::SystemTime;
use crate::domain::value_object::{email::Email, validation_code::ValidationCode};

#[derive(Debug, Clone)]
pub struct ValidationCodeRecord {
    pub email: Email,
    pub code: ValidationCode,
    pub expires_at: SystemTime,
}

impl ValidationCodeRecord {
    pub fn new(email: Email, code: ValidationCode, ttl_secs: u64) -> Self {
        let expires_at = SystemTime::now() + std::time::Duration::from_secs(ttl_secs);
        Self {
            email,
            code,
            expires_at,
        }
    }

    pub fn from_trusted(email: Email, code: ValidationCode, expires_at: SystemTime) -> Self {
        Self {
            email,
            code,
            expires_at,
        }
    }

    /// 核心业务逻辑：校验验证码是否匹配且未过期
    pub fn is_valid(&self, attempt: &ValidationCode) -> bool {
        !self.is_expired() && &self.code == attempt
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}
