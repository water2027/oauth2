use crate::context::oauth::{error::DomainError, value_object::{ClientID, HashedSecret, RedirectUri}};


pub struct Client {
    pub id: ClientID,
    pub secret: HashedSecret,
    pub redirect_uris: Vec<RedirectUri>,
}

impl Client {
    pub fn new(id: ClientID, secret: HashedSecret, redirect_uris: Vec<RedirectUri>) -> Self {
        Self { id, secret, redirect_uris }
    }
    
    pub fn reset_secret(&mut self, new_secret: HashedSecret) {
        self.secret = new_secret;
    }
    
    pub fn verify_redirect_uri(&self, target: &RedirectUri) -> bool {
        self.redirect_uris.iter().any(|uri| uri == target)        
    }
    
    pub fn add_new_uri(&mut self, uri: RedirectUri) -> Result<(), DomainError> {
        if self.redirect_uris.len() > 4 {
            return Err(DomainError::TooManyRedirectUri)
        }
        self.redirect_uris.push(uri);
        Ok(())
    }
    
    pub fn remove_uri(&mut self, uri: &RedirectUri) -> Result<(), DomainError> {
        if let Some(pos) = self.redirect_uris.iter().position(|x| x == uri) {
            self.redirect_uris.remove(pos);
            Ok(())
        } else {
            Err(DomainError::InvalidRedirectUriCount)
        }
    }
}