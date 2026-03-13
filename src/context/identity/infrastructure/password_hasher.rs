use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::context::identity::value_object::{HashedPassword, RawPassword};
use crate::context::identity::error::DomainError;
use crate::context::identity::service::password_hasher::IPasswordHasher;

pub struct Argon2PasswordHasher;

impl IPasswordHasher for Argon2PasswordHasher {
    fn hash(&self, raw: RawPassword) -> Result<HashedPassword, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(raw.as_ref().as_bytes(), &salt)
            .map_err(|e| DomainError::InternalError(format!("Hashing failed: {}", e)))?
            .to_string();
        Ok(HashedPassword::from_trusted(password_hash))
    }

    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hashed.as_ref())
            .map_err(|e| DomainError::InternalError(format!("Invalid hash: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(raw.as_ref().as_bytes(), &parsed_hash)
            .is_ok())
    }
}
