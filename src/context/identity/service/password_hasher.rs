use super::super::value_object::{hashed_password::HashedPassword, raw_password::RawPassword};
use crate::shared::domain::error::DomainError;

pub trait IPasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, DomainError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> Result<bool, DomainError>;
}
