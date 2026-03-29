use async_trait::async_trait;
use crate::shared::error::RepositoryError;
use super::super::entity::Client;
use super::super::value_object::ClientID;

#[async_trait]
pub trait IClientRepository: Send + Sync {
    async fn find_by_id(&self, id: &ClientID) -> Result<Option<Client>, RepositoryError>;
}
