use crate::domain::error::DomainError;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationCode (String);

impl AsRef<str> for ValidationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ValidationCode {
    pub fn parse(code: String) -> Result<Self, DomainError> {
        Ok(ValidationCode(code))
    }

    pub fn from_trusted(code: String) -> Self {
        ValidationCode(code)
    }
    
    pub fn verify(&self, code: String) -> bool {
        code == self.0
    }
}
