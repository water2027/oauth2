use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword (String);

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl HashedPassword {
    pub fn new(pass: String) -> Result<Self, DomainError> {
        Ok(HashedPassword(pass))
    }

    pub fn from_trusted(plaintext: String) -> Self {
        HashedPassword(plaintext)
    }
}
