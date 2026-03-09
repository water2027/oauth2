use async_trait::async_trait;
use super::super::entity::session::Session;

use crate::shared::error::RepositoryError;

#[async_trait]
pub trait SessionRepository {
    async fn generate_cookie(&self) -> Result<String, RepositoryError>;
    async fn get_session(&self, cookie: &str) -> Result<Option<Session>, RepositoryError>;
    async fn delete_session(&self, cookie: &str) -> Result<(), RepositoryError>;
    async fn save_session(&self, session: &Session) -> Result<(), RepositoryError>;
}