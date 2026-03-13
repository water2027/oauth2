use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedSecret (String);

impl AsRef<str> for HashedSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl HashedSecret {
    pub fn new(secret: String) -> Result<Self, DomainError> {
        Ok(HashedSecret(secret))
    }

    pub fn from_trusted(secret: String) -> Self {
        HashedSecret(secret)
    }
}
