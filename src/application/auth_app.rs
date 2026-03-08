use crate::{application::{command::{LoginCommand, RegisterCommand}, error::AppError}, domain::{entity::user::User, repository::user::IUserRepository, service::{code::CodeService, password_hasher::IPasswordHasher}, value_object::user_id::UserID}};


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
        let user = self.user_repo.find_by_email(&email).await?;
        if user.is_none() {
            return Err(AppError::InvalidCredentials);
        }
        if !self.password_hasher.verify(&password, &user.unwrap().password)? {
            return Err(AppError::InvalidCredentials);
        }

        // TODO: 颁发token

        Ok(())
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<(), AppError> {
        // TODO: 开启事务
        let RegisterCommand { username, email, password, password_confirm, validation_code } = cmd;
        if self.user_repo.is_email_registered(&email).await? {
            // TODO: do sth
        }

        if password != password_confirm {
            // TODO: do sth
            return Err(AppError::InvalidCredentials);
        }

        self.code_service.verify_code(&email, &validation_code).await?;

        // need id generator

        let user_id = UserID::from_trusted("user_id".to_string());
        let new_user = User::new(user_id, username, email, self.password_hasher.hash(&password)?);

        self.user_repo.create(&new_user).await?;

        self.user_repo.save(&new_user).await?;

        // token

        Ok(())
    }


}
