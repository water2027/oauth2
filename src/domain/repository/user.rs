use crate::domain::{entity::user::User, error::RepositoryError, value_object::{email::Email, user_id::UserID}};

pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: &UserID) -> Result<User, RepositoryError>;
    async fn is_email_registered(&self, email: &Email) -> Result<bool, RepositoryError>;
    async fn create(&self, user: &User) -> Result<(), RepositoryError>;
    async fn save(&self, uesr: &User) -> Result<(), RepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<User, RepositoryError>;
}