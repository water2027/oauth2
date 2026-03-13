use super::super::service::password_hasher::{IPasswordHasher};
use std::sync::Arc;
use super::super::{entity::User, repository::user::{IUserRepository, IUserIDGenerator}, value_object::{Email, RawPassword, Username}, error::DomainError};


pub struct UserService {
    user_repository: Arc<dyn IUserRepository>,
    user_id_generator: Arc<dyn IUserIDGenerator>,
    password_hasher: Arc<dyn IPasswordHasher>
}

impl UserService {
    pub fn new(user_repository: Arc<dyn IUserRepository>, user_id_generator: Arc<dyn IUserIDGenerator>, password_hasher: Arc<dyn IPasswordHasher>) -> Self {
        Self { user_repository, user_id_generator, password_hasher }
    }

    pub async fn create_user(&self, email: Email, password: RawPassword, username: Username) -> Result<User, DomainError> {
        let user_id = self.user_id_generator.generate_id();
        let hashed_password = self.password_hasher.hash(password)?;
        let user = User::new(user_id, username, email, hashed_password);
        self.user_repository.create(&user).await?;
        Ok(user)
    }
    
    pub async fn reset_password(&self, email: &Email, new_pass: RawPassword) -> Result<(), DomainError> {
        let user = self.user_repository.find_by_email(&email).await?;
        if let Some(mut user) = user {
            let hashed_password = self.password_hasher.hash(new_pass)?;
            user.password = hashed_password;
            self.user_repository.save(&user).await?;
            Ok(())
        } else {
            Err(DomainError::UserNotFound)
        }
    }
    
    pub async fn can_create(&self, email: &Email, password: &RawPassword, password_confirm: &RawPassword) -> Result<(), DomainError> {
        if password != password_confirm {
            return Err(DomainError::PasswordMismatch);
        }
        if self.user_repository.find_by_email(email).await?.is_some() {
            return Err(DomainError::EmailAlreadyExists);
        }
        Ok(())
    }

    pub async fn find_user_by_email(&self, email: Email) -> Result<Option<User>, DomainError> {
        let user = self.user_repository.find_by_email(&email).await?;
        Ok(user)
    }
    
    pub async fn authenticate(&self, email: Email, password: RawPassword) -> Result<User, DomainError> {
        let user = self.user_repository.find_by_email(&email).await?;
        if let Some(user) = user {
            if self.password_hasher.verify(&password, &user.password)? {
                Ok(user)
            } else {
                Err(DomainError::PasswordMismatch)
            }
        } else {
            Err(DomainError::UserNotFound)
        }
    }
}
