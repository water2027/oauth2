use std::sync::Arc;
use crate::context::oauth::{
    error::DomainError,
    value_object::{AuthTicket, ClientID, PkceInfo, RedirectUri, Scope},
    aggregate::session::AuthorizationSession,
    repository::{IClientRepository, IAuthorizationSessionRepository},
    service::IAuthTicketGenerator,
};

pub struct StartSessionCommand {
    pub state: String,
    pub pkce: PkceInfo,
    pub redirect_uri: RedirectUri,
    pub scope: Scope,
    pub client_id: ClientID,
}

pub struct StartSessionUseCase {
    client_repo: Arc<dyn IClientRepository>,
    session_repo: Arc<dyn IAuthorizationSessionRepository>,
    ticket_generator: Arc<dyn IAuthTicketGenerator>,
}

impl StartSessionUseCase {
    pub fn new(
        client_repo: Arc<dyn IClientRepository>,
        session_repo: Arc<dyn IAuthorizationSessionRepository>,
        ticket_generator: Arc<dyn IAuthTicketGenerator>,
    ) -> Self {
        Self {
            client_repo,
            session_repo,
            ticket_generator,
        }
    }

    pub async fn execute(&self, cmd: StartSessionCommand) -> Result<AuthTicket, DomainError> {
        // 1. Verify client exists and redirect_uri is valid
        let client = self.client_repo.find_by_id(&cmd.client_id).await?;
        let client = client.ok_or(DomainError::InvalidClient)?;

        if !client.verify_redirect_uri(&cmd.redirect_uri) {
            return Err(DomainError::InvalidRedirectUri);
        }

        // 2. Generate AuthTicket
        let ticket = self.ticket_generator.generate();

        // 3. Create AuthorizationSession
        let session = AuthorizationSession::new(
            ticket.clone(),
            cmd.client_id,
            cmd.pkce,
            cmd.redirect_uri,
            cmd.scope,
            cmd.state,
        );

        // 4. Save session
        self.session_repo.save(&session).await?;

        Ok(ticket)
    }
}
