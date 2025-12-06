pub mod aes;
pub mod bcrypt;

pub use aes::AesGcmEncryptor;
pub use bcrypt::BcryptHasher;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Password hashing failed: {0}")]
    HashingError(String),
    #[error("Password verification failed: {0}")]
    VerificationError(String),
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}

pub trait PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, CryptoError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, CryptoError>;
}

pub trait Encryptor {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, CryptoError>;
}