use serde::{Serialize, Deserialize};
use super::super::error::DomainError;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email (String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn parse(address: String) -> Result<Self, DomainError> {
        if address.contains('@') {
            Ok(Email(address))
        } else {
            Err(DomainError::InvalidEmailFormat)
        }
    }
    
    pub fn from_trusted(email: String) -> Self {
        Email(email)
    }
}