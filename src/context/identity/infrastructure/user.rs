use super::super::entity::User;
use super::super::value_object::{Email, UserID};
use crate::context::identity::repository::user::{IUserIDGenerator, IUserRepository};
use crate::context::identity::value_object::HashedPassword;
use crate::context::identity::value_object::Username;
use crate::shared::error::RepositoryError;
use async_trait::async_trait;
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
            let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email.as_ref())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
            Ok(row.0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::env;

    async fn setup_pool() -> PgPool {
        // 加载 .env 环境变量
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.expect("Failed to connect to Postgres")
    }

    #[tokio::test]
    async fn test_user_repository_flow() {
        let pool = setup_pool().await;
        let repo = SqlxUserRepository::new(pool);

        let user_id = UserID::from_trusted(Uuid::new_v4().to_string());
        let username = Username::from_trusted("testuser".to_string());
        let email = Email::from_trusted(format!("test-{}@example.com", Uuid::new_v4()));
        let password = HashedPassword::from_trusted("hashed_pwd".to_string());

        let user = User::new(user_id.clone(), username.clone(), email.clone(), password.clone());

        // 1. 测试创建 (Create)
        repo.create(&user).await.expect("Failed to create user");

        // 2. 测试按 ID 查找 (Find by ID)
        let found = repo.find_by_id(&user_id).await.expect("Failed to find user by ID");
        assert!(found.is_some(), "User should be found by ID");
        let found = found.unwrap();
        assert_eq!(found.username.as_ref(), "testuser");
        assert_eq!(found.email.as_ref(), email.as_ref());

        // 3. 测试邮箱是否已注册 (验证 EXISTS 逻辑)
        let registered = repo.is_email_registered(&email).await.expect("Failed to check email");
        assert!(registered, "Email should be registered");

        // 4. 测试按邮箱查找 (Find by Email)
        let found_by_email = repo.find_by_email(&email).await.expect("Failed to find user by email");
        assert!(found_by_email.is_some(), "User should be found by email");

        // 5. 测试更新 (Save/Update)
        let mut user_to_update = found;
        let new_username = Username::from_trusted("updated_name".to_string());
        user_to_update.username = new_username.clone();
        repo.save(&user_to_update).await.expect("Failed to update user");

        let updated = repo.find_by_id(&user_id).await.expect("Failed to find updated user").unwrap();
        assert_eq!(updated.username.as_ref(), "updated_name");
    }
}