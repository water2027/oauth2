use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawSecret (String);

impl AsRef<str> for RawSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl RawSecret {
    pub fn parse(plaintext: String) -> Result<Self, DomainError> {
        Ok(RawSecret(plaintext))
    }

    pub fn from_trusted(plaintext: String) -> Self {
        RawSecret(plaintext)
    }
}
