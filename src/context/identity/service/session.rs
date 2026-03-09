use std::sync::Arc;
use crate::context::identity::{
    entity::session::{Session},
    error::DomainError,
    repository::session::SessionRepository,
    value_object::user_id::UserID,
};

pub struct SessionService {
    session_repository: Arc<dyn SessionRepository>
}

impl SessionService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }

    pub async fn create_session(&self, user_id: UserID) -> Result<Session, DomainError> {
        let cookie = self.session_repository.generate_cookie().await?;
        let session = Session::new(user_id, cookie);
        self.session_repository.save_session(&session).await?;
        Ok(session)
    }
    pub async fn verify_session(&self, cookie: &str) -> Result<Option<Session>, DomainError> {
        let session = self.session_repository.get_session(cookie).await?;
        if let Some(ref s) = session {
            if s.is_expired() {
                self.session_repository.delete_session(cookie).await?;
                return Ok(None);
            }
        }
        Ok(session)
    }
    pub async fn refresh_session(&self, session: &mut Session) -> Result<(), DomainError> {
        if session.refresh() {
            self.session_repository.save_session(session).await?;
        }
        Ok(())
    }
}