use super::command::{LoginCommand, RegisterCommand};
use super::error::AppError;
use super::super::entity::user::User;
use super::super::repository::user::IUserRepository;
use super::super::service::{code::CodeService, password_hasher::IPasswordHasher};
use super::super::value_object::user_id::UserID;
use crate::shared::domain::error::DomainError;

pub struct AuthAppService {
    user_repo: Box<dyn IUserRepository>,
    code_service: CodeService,
    password_hasher: Box<dyn IPasswordHasher>,
}

impl AuthAppService {
    pub fn new(user_repo: Box<dyn IUserRepository>, code_service: CodeService, password_hasher: Box<dyn IPasswordHasher>) -> Self {
        Self { user_repo, code_service, password_hasher }
    }
    
    pub async fn login(&self, cmd: LoginCommand) -> Result<(), AppError> {
        let LoginCommand { email, password } = cmd;
        let user = self.user_repo.find_by_email(&email).await.map_err(DomainError::Repository)?;
        if user.is_none() {
            return Err(AppError::InvalidCredentials);
        }
        if !self.password_hasher.verify(&password, &user.unwrap().password)? {
            return Err(AppError::InvalidCredentials);
        }

        // TODO: 颁发 cookie session

        Ok(())
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<(), AppError> {
        let RegisterCommand { username, email, password, password_confirm, validation_code } = cmd;
        if self.user_repo.is_email_registered(&email).await.map_err(DomainError::Repository)? {
            // handle error
        }

        if password != password_confirm {
            return Err(AppError::InvalidCredentials);
        }

        self.code_service.verify_code(&email, &validation_code).await?;

        let user_id = UserID::from_trusted("user_id".to_string());
        let new_user = User::new(user_id, username, email, self.password_hasher.hash(&password)?);

        self.user_repo.create(&new_user).await.map_err(DomainError::Repository)?;

        Ok(())
    }
}
