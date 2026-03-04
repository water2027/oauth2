use crate::domain::value_object::{email::Email, hashed_password::HashedPassword, username::Username};

pub struct User {
    pub user_id: String,
    pub username: Username,
    pub email: Email,
    pub password: HashedPassword
}

impl User {
    pub fn new(username: Username, email: Email, password: HashedPassword) -> Self {
        User { username, email, password }
    }
    
    pub fn set_password(&mut self, password: HashedPassword) -> bool {
        self.password = password;
        true
    }
}
