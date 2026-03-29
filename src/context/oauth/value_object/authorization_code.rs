use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationCode (String);

impl AsRef<str> for AuthorizationCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AuthorizationCode {
    pub fn parse(code: String) -> Result<Self, DomainError> {
        Ok(AuthorizationCode(code))
    }

    pub fn from_trusted(code: String) -> Self {
        AuthorizationCode(code)
    }
}
