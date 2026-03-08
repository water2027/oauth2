use crate::shared::domain::error::DomainError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username (String);

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Username {
    pub fn parse(name: String) -> Result<Self, DomainError> {
        Ok(Username(name))
    }

    pub fn from_trusted(name: String) -> Self {
        Username(name)
    }
}
