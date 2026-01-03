use super::{CryptoError, PasswordHasher};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct BcryptHasher;

impl PasswordHasher for BcryptHasher {
    fn hash_password(&self, password: &str) -> Result<String, CryptoError> {
        hash(password, DEFAULT_COST).map_err(|e| CryptoError::HashingError(e.to_string()))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, CryptoError> {
        verify(password, hash).map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password_success() {
        let hasher = BcryptHasher;
        let password = "my_secure_password";

        let hashed_password = hasher.hash_password(password).unwrap();

        assert_ne!(password, hashed_password);

        let is_valid = hasher.verify_password(password, &hashed_password).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_password_incorrect_password() {
        let hasher = BcryptHasher;
        let password = "my_secure_password";
        let incorrect_password = "incorrect_password";

        let hashed_password = hasher.hash_password(password).unwrap();

        let is_valid = hasher
            .verify_password(incorrect_password, &hashed_password)
            .unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_verify_password_malformed_hash() {
        let hasher = BcryptHasher;
        let password = "my_secure_password";
        let malformed_hash = "not_a_real_hash";

        let result = hasher.verify_password(password, malformed_hash);
        assert!(result.is_err());

        match result.unwrap_err() {
            CryptoError::VerificationError(_) => (), // Expected error
            _ => panic!("Expected VerificationError for malformed hash"),
        }
    }
}
