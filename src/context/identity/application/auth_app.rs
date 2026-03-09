use crate::context::identity::service::user::UserService;
use super::super::error::DomainError;

use super::command::{LoginCommand, RegisterCommand};
use super::super::service::{code::CodeService};

pub struct AuthAppService {
    user_service: UserService,
    code_service: CodeService,
}

impl AuthAppService {
    pub fn new(user_service: UserService, code_service: CodeService) -> Self {
        Self { user_service, code_service }
    }

    pub async fn login(&self, cmd: LoginCommand) -> Result<(), DomainError> {
        let LoginCommand { email, password } = cmd;

        self.user_service.authenticate(email, password).await?;
        
        // TODO: 颁发 cookie session

        Ok(())
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<(), DomainError> {
        let RegisterCommand { username, email, password, password_confirm, validation_code } = cmd;

        self.code_service.verify_code(&email, &validation_code).await?;

        self.user_service.can_create(&email, &password, &password_confirm).await?;
        self.user_service.create_user(email, password, username).await?;

        // TODO: 颁发 cookie session
        
        Ok(())
    }
}
