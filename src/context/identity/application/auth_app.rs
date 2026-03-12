use crate::context::identity::entity::session::Session;
use crate::context::identity::service::session::SessionService;
use crate::context::identity::service::user::UserService;
use crate::context::identity::value_object::email::Email;
use super::super::error::DomainError;

use super::command::{LoginCommand, RegisterCommand};
use super::super::service::{code::CodeService};

pub struct AuthAppService {
    session_service: SessionService,
    user_service: UserService,
    code_service: CodeService,
}

impl AuthAppService {
    pub fn new(session_service: SessionService, user_service: UserService, code_service: CodeService) -> Self {
        Self { session_service, user_service, code_service }
    }

    pub async fn login(&self, cmd: LoginCommand) -> Result<Session, DomainError> {
        let LoginCommand { email, password } = cmd;

        let user = self.user_service.authenticate(email, password).await?;
        
        let session = self.session_service.create_session(user.user_id).await?;
        
        Ok(session)
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<Session, DomainError> {
        let RegisterCommand { username, email, password, password_confirm, validation_code } = cmd;

        if !self.code_service.verify_code(&email, &validation_code).await? {
            return Err(DomainError::InvalidValidationCode);
        }

        self.user_service.can_create(&email, &password, &password_confirm).await?;
        let user = self.user_service.create_user(email, password, username).await?;

        let session = self.session_service.create_session(user.user_id).await?;

        Ok(session)
    }
    
    pub async fn logout(&self, cookie: &str) -> Result<(), DomainError> {
        self.session_service.delete_session(cookie).await?;
        Ok(())
    }
    
    pub async fn send_validation_code(&self, email: Email) -> Result<(), DomainError> {
        if !self.code_service.can_send_code(&email).await? {
            return Err(DomainError::TooManyRequests);
        }
        self.code_service.send_code(&email).await?;
        Ok(())
    }
    
    pub async fn verify(&self, cookie: &str) -> Result<Option<Session>, DomainError> {
        let session = self.session_service.verify_session(cookie).await?;
        Ok(session)
    }
}
