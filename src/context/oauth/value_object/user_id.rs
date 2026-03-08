use crate::shared::domain::error::DomainError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID (String);

impl AsRef<str> for UserID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl UserID {
    pub fn parse(id: String) -> Result<Self, DomainError> {
        Ok(UserID(id))
    }

    pub fn from_trusted(id: String) -> Self {
        UserID(id)
    }
}
