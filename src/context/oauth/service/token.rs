use super::super::value_object::user_id::UserID;

pub struct TokenService {
    
}

impl TokenService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify_token(&self, _token: &str) -> bool {
        todo!("Implement token verification")
    }

    pub fn issue_token(&self, _user_id: UserID) {
        todo!("Implement token issuance")
    }

    pub fn revoke_token(&self, _token: &str) {
        todo!("Implement token revocation")
    }

    pub fn refresh_token(&self, _token: &str) {
        todo!("Implement token refresh")
    }
}
