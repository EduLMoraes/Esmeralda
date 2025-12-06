use super::{CryptoError, PasswordHasher};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct BcryptHasher;

impl PasswordHasher for BcryptHasher {
    fn hash_password(&self, password: &str) -> Result<String, CryptoError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| CryptoError::HashingError(e.to_string()))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, CryptoError> {
        verify(password, hash)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}
