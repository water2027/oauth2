use crate::domain::value_object::user_id::UserID;

pub struct TokenService {
    
}

impl TokenService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify_token(&self, token: &str) -> bool {

    }

    pub fn issue_token(&self, user_id: UserID) -> todo!("Implement token issuance") {

    }

    pub fn revoke_token(&self, token: &str) -> todo!("Implement token revocation") {

    }

    pub fn refresh_token(&self, token: &str) -> todo!("Implement token refresh") {

    }
}