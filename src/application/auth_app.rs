use crate::{application::{command::{LoginCommand, RegisterCommand}, error::AppError}, domain::{entity::user::User, repository::user::UserRepository, service::password_hasher::PasswordHasher}};


pub struct AuthAppService<R, H> {
    user_repo: R,
    password_hasher: H,
}

impl<R, H> AuthAppService<R, H>
where
    R: UserRepository + Send + Sync,
    H: PasswordHasher + Send + Sync
{
    pub fn new(user_repo: R, password_hasher: H) -> Self {
        Self { user_repo, password_hasher }
    }
    pub async fn login(&self, cmd: LoginCommand) -> Result<(), AppError> {
        let LoginCommand { email, password } = cmd;
        let user = self.user_repo.find_by_email(&email).await?;
        if !self.password_hasher.verify(&password, &user.password)? {
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
        }
        
        // need code_repo
        
        let new_user = User::new(username, email, self.password_hasher.hash(&password)?);
        
        self.user_repo.create(&new_user).await?;
        
        self.user_repo.save(&new_user).await?;
        
        // token
        
        Ok(())
    }
    
    
}
