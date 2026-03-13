use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use super::super::value_object::{Email, ValidationCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCodeRecord {
    pub email: Email,
    pub code: ValidationCode,
    pub expires_at: DateTime<Utc>,
}

impl ValidationCodeRecord {
    pub fn new(email: Email, code: ValidationCode, ttl_secs: u64) -> Self {
        let expires_at = Utc::now() + Duration::seconds(ttl_secs as i64);
        Self {
            email,
            code,
            expires_at,
        }
    }

    pub fn from_trusted(email: Email, code: ValidationCode, expires_at: DateTime<Utc>) -> Self {
        Self {
            email,
            code,
            expires_at,
        }
    }

    pub fn is_valid(&self, attempt: &ValidationCode) -> bool {
        !self.is_expired() && &self.code == attempt
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
