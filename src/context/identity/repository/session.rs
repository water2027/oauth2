use async_trait::async_trait;
use super::super::entity::session::Session;
use super::super::value_object::user_id::UserID;

use crate::shared::error::RepositoryError;

#[async_trait]
pub trait ISessionRepository: Send + Sync {
    async fn generate_cookie(&self) -> Result<String, RepositoryError>;
    async fn get_session(&self, cookie: &str) -> Result<Option<Session>, RepositoryError>;
    async fn delete_session(&self, cookie: &str) -> Result<(), RepositoryError>;
    async fn save_session(&self, session: &Session) -> Result<(), RepositoryError>;
    async fn delete_user_sessions(&self, user_id: &UserID) -> Result<(), RepositoryError>;
}