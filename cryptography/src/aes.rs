use super::{CryptoError, Encryptor};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use std::env;

pub struct AesGcmEncryptor {
    key: [u8; 32],
}

impl AesGcmEncryptor {
    pub fn new() -> Result<Self, CryptoError> {
        // FIXME: Using environment variables for keys is insecure.
        // This should be replaced with a more secure key management solution.
        let key_str = env::var("KEYESMERALD")
            .map_err(|_| CryptoError::InvalidKey("KEYESMERALD environment variable not set".to_string()))?;
        let key_bytes = key_str.as_bytes();

        if key_bytes.len() != 32 {
            return Err(CryptoError::InvalidKey("Key must be 32 bytes long".to_string()));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(key_bytes);

        Ok(Self { key })
    }
}

impl Encryptor for AesGcmEncryptor {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new(&self.key.into());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if encrypted_data.len() < 12 {
            return Err(CryptoError::DecryptionError("Invalid encrypted data".to_string()));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new(&self.key.into());
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))
    }
}
