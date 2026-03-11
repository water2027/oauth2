use super::super::entity::user::User;
use super::super::value_object::{email::Email, user_id::UserID};
use crate::context::identity::repository::user::{IUserIDGenerator, IUserRepository};
use crate::context::identity::value_object::hashed_password::HashedPassword;
use crate::context::identity::value_object::username::Username;
use crate::shared::error::RepositoryError;
use async_trait::async_trait;
use sqlx::types::uuid;
use uuid::Uuid;

pub struct SqlxUserRepository {
    pool: sqlx::PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    username: String,
    email: String,
    password: String,
}

#[async_trait]
impl IUserRepository for SqlxUserRepository {
    async fn find_by_id(&self, user_id: &UserID) -> Result<Option<User>, RepositoryError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT id, username, email, password FROM users WHERE id = $1")
            .bind(user_id.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        if let Some(user) = row {
            Ok(Some(User {
                user_id: UserID::from_trusted(user.id),
                username: Username::from_trusted(user.username),
                email: Email::from_trusted(user.email),
                password: HashedPassword::from_trusted(user.password),
            }))
        } else {
            Ok(None)
        }
    }
    async fn is_email_registered(&self, email: &Email) -> Result<bool, RepositoryError> {
            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1")
                .bind(email.as_ref())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
            Ok(count.0 > 0)
    }
    async fn create(&self, user: &User) -> Result<(), RepositoryError> {
            sqlx::query("INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)")
                .bind(user.user_id.as_ref())
                .bind(user.username.as_ref())
                .bind(user.email.as_ref())
                .bind(user.password.as_ref())
                .execute(&self.pool)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
            Ok(())
    }
    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
            sqlx::query("UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4")
                .bind(user.username.as_ref())
                .bind(user.email.as_ref())
                .bind(user.password.as_ref())
                .bind(user.user_id.as_ref())
                .execute(&self.pool)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
            Ok(())
    }
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        let row: Option<UserRow> = sqlx::query_as("SELECT id, username, email, password FROM users WHERE email = $1")
            .bind(email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        if let Some(user) = row {
            Ok(Some(User {
                user_id: UserID::from_trusted(user.id),
                username: Username::from_trusted(user.username),
                email: Email::from_trusted(user.email),
                password: HashedPassword::from_trusted(user.password),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct UUIDUserIDGenerator;

impl IUserIDGenerator for UUIDUserIDGenerator {
    fn generate_id(&self) -> UserID {
        let id = Uuid::new_v4().to_string();
        UserID::from_trusted(id)
    }
}