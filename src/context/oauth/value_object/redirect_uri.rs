use super::super::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedirectUri (String);

impl AsRef<str> for RedirectUri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl RedirectUri {
    pub fn parse(uri: String) -> Result<Self, DomainError> {
        if !uri.starts_with("https") || !uri.starts_with("http://localhost") || !uri.starts_with("http://127.0.0.1") {
            return Err(DomainError::UnsafeRedirectUri)
        }
        Ok(RedirectUri(uri))
    }

    pub fn from_trusted(uri: String) -> Self {
        RedirectUri(uri)
    }
}
