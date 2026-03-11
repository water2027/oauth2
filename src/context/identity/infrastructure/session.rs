use async_trait::async_trait;
use super::super::entity::session::Session;

use crate::{context::identity::{repository::session::SessionRepository, value_object::user_id::UserID}, shared::{error::RepositoryError, utils::generate_random_string}};

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
impl SessionRepository for SqlxSessionRepository {
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
        sqlx::query("INSERT INTO sessions (cookie, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(&session.cookie)
            .bind(&session.user_id.as_ref())
            .bind(session.expires_at)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("sqlx: {}", e.to_string())))?;
        Ok(())
    }
}