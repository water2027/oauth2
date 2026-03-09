use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationCode(String);

impl AsRef<str> for ValidationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ValidationCode {
    pub fn new(code: String) -> Result<Self, DomainError> {
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::InternalError("验证码格式错误".to_string()));
        }
        Ok(ValidationCode(code))
    }

    pub fn from_trusted(code: String) -> Self {
        ValidationCode(code)
    }
}
