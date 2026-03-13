use super::super::value_object::{HashedPassword, RawPassword};
use super::super::error::DomainError;

pub trait IPasswordHasher: Send + Sync {
    fn hash(&self, raw: RawPassword) -> Result<HashedPassword, DomainError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> Result<bool, DomainError>;
}
