use async_trait::async_trait;
use crate::shared::error::RepositoryError;
use super::super::aggregate::session::AuthorizationSession;

#[async_trait]
pub trait IAuthorizationSessionRepository: Send + Sync {
    async fn save(&self, session: &AuthorizationSession) -> Result<(), RepositoryError>;
}
