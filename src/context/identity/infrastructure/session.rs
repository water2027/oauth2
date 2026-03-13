use async_trait::async_trait;
use super::super::entity::Session;

use crate::{context::identity::{repository::session::ISessionRepository, value_object::UserID}, shared::{error::RepositoryError, utils::generate_random_string}};

pub struct SqlxSessionRepository {
    pool: sqlx::PgPool,
}

impl SqlxSessionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct SessionRow {
    user_id: String,
    expires_at: i64,
}

#[async_trait]
impl ISessionRepository for SqlxSessionRepository {
    async fn generate_cookie(&self) -> Result<String, RepositoryError> {
        Ok(generate_random_string(32))
    }
    async fn get_session(&self, cookie: &str) -> Result<Option<Session>, RepositoryError> {
        let row: Option<SessionRow> = sqlx::query_as("SELECT user_id, expires_at FROM sessions WHERE cookie = $1")
            .bind(cookie)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        if let Some(row) = row {
            Ok(Some(Session {
                cookie: cookie.to_string(),
                user_id: UserID::from_trusted(row.user_id),
                expires_at: row.expires_at,
            }))
        } else {
            Ok(None)
        }
    }
    async fn delete_session(&self, cookie: &str) -> Result<(), RepositoryError> {
            sqlx::query("DELETE FROM sessions WHERE cookie = $1")
                .bind(cookie)
                .execute(&self.pool)
                .await
                .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
            Ok(())
    }
    async fn save_session(&self, session: &Session) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO sessions (cookie, user_id, expires_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (cookie)
            DO UPDATE SET expires_at = EXCLUDED.expires_at
            "#
        )
        .bind(&session.cookie)
        .bind(&session.user_id.as_ref())
        .bind(session.expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        Ok(())
    }
    
    async fn delete_user_sessions(&self, user_id: &UserID) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id.as_ref())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::env;
    use uuid::Uuid;

    async fn setup_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.expect("Failed to connect to Postgres")
    }

    #[tokio::test]
    async fn test_session_repository_flow() {
        let pool = setup_pool().await;
        let session_repo = SqlxSessionRepository::new(pool.clone());
        
        // 1. 创建一个测试用户 (因为 Session 有外键约束)
        let user_id = UserID::from_trusted(Uuid::new_v4().to_string());
        let email = format!("session-test-{}@example.com", Uuid::new_v4());
        sqlx::query("INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)")
            .bind(user_id.as_ref())
            .bind("testuser")
            .bind(&email)
            .bind("hashed_pwd")
            .execute(&pool)
            .await
            .expect("Failed to create test user for session test");

        // 2. 生成并保存 Session
        let cookie = session_repo.generate_cookie().await.expect("Failed to generate cookie");
        let session = Session::new(user_id.clone(), cookie.clone());
        session_repo.save_session(&session).await.expect("Failed to save session");

        // 3. 获取 Session
        let found = session_repo.get_session(&cookie).await.expect("Failed to get session");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.user_id.as_ref(), user_id.as_ref());
        assert_eq!(found.cookie, cookie);

        // 4. 删除 Session
        session_repo.delete_session(&cookie).await.expect("Failed to delete session");
        let deleted = session_repo.get_session(&cookie).await.expect("Failed to check deleted session");
        assert!(deleted.is_none());
    }
}