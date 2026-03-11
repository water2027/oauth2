use super::super::entity::user::User;
use super::super::value_object::{email::Email, user_id::UserID};
use crate::shared::error::RepositoryError;
use async_trait::async_trait;

pub trait IUserIDGenerator {
    fn generate_id(&self) -> UserID;
}

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: &UserID) -> Result<Option<User>, RepositoryError>;
    async fn is_email_registered(&self, email: &Email) -> Result<bool, RepositoryError>;
    async fn create(&self, user: &User) -> Result<(), RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
}
