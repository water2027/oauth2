use crate::domain::{error::DomainError, value_object::{hashed_password::HashedPassword, raw_password::RawPassword}};

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, DomainError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> Result<bool, DomainError>;
}
