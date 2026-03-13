use super::super::value_object::{Email, HashedPassword,Username, UserID};

pub struct User {
    pub user_id: UserID,
    pub username: Username,
    pub email: Email,
    pub password: HashedPassword
}

impl User {
    pub fn new(user_id: UserID, username: Username, email: Email, password: HashedPassword) -> Self {
        User { user_id, username, email, password }
    }
    
    pub fn set_password(&mut self, password: HashedPassword) -> bool {
        self.password = password;
        true
    }
}
