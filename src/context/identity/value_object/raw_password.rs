use crate::shared::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPassword (String);

impl AsRef<str> for RawPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl RawPassword {
    pub fn new(plaintext: String) -> Result<Self, DomainError> {
        if plaintext.len() < 6 {
            return Err(DomainError::WeakPassword);
        }
        // TODO: 校验复杂度

        Ok(RawPassword(plaintext))
    }

    pub fn from_trusted(plaintext: String) -> Self {
        RawPassword(plaintext)
    }
}
