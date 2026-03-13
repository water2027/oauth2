use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientID (String);

impl AsRef<str> for ClientID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ClientID {
    pub fn parse(id: String) -> Result<Self, DomainError> {
        Ok(ClientID(id))
    }

    pub fn from_trusted(id: String) -> Self {
        ClientID(id)
    }
}
