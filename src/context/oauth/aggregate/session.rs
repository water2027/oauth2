use crate::context::oauth::{entity::TokenPair, error::DomainError, value_object::{AuthTicket, AuthorizationCode, ClientID, PkceInfo, UserID, RedirectUri, Scope}};

#[derive(Debug, PartialEq)]
pub enum SessionStatus {
    Pending,
    Authorized,
    CodeIssued,
    TokenExpired,
    TokenExchanged
}

pub struct AuthorizationSession {
    pub ticket: AuthTicket,
    pub client_id: ClientID,
    pub user_id: Option<UserID>,
    pub pkce_info: PkceInfo,
    pub redirect_uri: RedirectUri,
    pub scope: Scope,
    pub state: String,
    pub authorization_code: Option<AuthorizationCode>, 
    pub status: SessionStatus,
}

impl AuthorizationSession {
    pub fn new(ticket: AuthTicket, client_id: ClientID, pkce_info: PkceInfo, redirect_uri: RedirectUri, scope: Scope, state: String) -> Self {
        Self {
            ticket,
            client_id,
            user_id: None,
            pkce_info,
            redirect_uri,
            scope,
            state,
            authorization_code: None,
            status: SessionStatus::Pending,
        }
    }

    pub fn grant_by(&mut self, user_id: UserID) -> Result<(), DomainError> {
        self.user_id = Some(user_id);
        self.status = SessionStatus::Authorized;
        Ok(())
    }
    
    pub fn generate_code(&mut self) -> Result<(), DomainError> {
        if self.status != SessionStatus::Authorized {
            todo!("");
        }
        
        // self.authorization_code = Rand
        self.status = SessionStatus::CodeIssued;
        Ok(())
    }
    
    pub fn exchange_token(&mut self, code_verifier: String) -> Result<(), DomainError> {
        if self.status != SessionStatus::CodeIssued {
            todo!();
        }
        
        if !self.pkce_info.verify(code_verifier)? {
            todo!();
        }
        
        self.status = SessionStatus::TokenExchanged;
        Ok(())
    }
}