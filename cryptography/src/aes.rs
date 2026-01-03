use super::{CryptoError, Encryptor};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
#[derive(Debug)]
pub struct AesGcmEncryptor {
    key: [u8; 32],
}

impl AesGcmEncryptor {
    pub fn new(key: [u8; 32]) -> Result<Self, CryptoError> {
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
            return Err(CryptoError::DecryptionError(
                "Invalid encrypted data".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new(&self.key.into());
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str = "a_32_byte_long_secret_key_for_testing";

    fn setup() {
        unsafe { std::env::set_var("KEYESMERALD", TEST_KEY) };
    }

    fn teardown() {
        unsafe { std::env::remove_var("KEYESMERALD") };
    }

    #[test]
    fn test_new_aes_gcm_encryptor_success() {
        setup();
        assert!(AesGcmEncryptor::new([0; 32]).is_ok());
        teardown();
    }

    #[test]
    fn test_new_aes_gcm_encryptor_no_key() {
        teardown(); // Ensure the key is not set
        let result = AesGcmEncryptor::new([0; 32]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CryptoError::InvalidKey(msg) => {
                assert_eq!(msg, "KEYESMERALD environment variable not set")
            }
            _ => panic!("Expected InvalidKey error"),
        }
    }

    #[test]
    fn test_new_aes_gcm_encryptor_invalid_key_length() {
        unsafe { std::env::set_var("KEYESMERALD", "short_key") };
        let result = AesGcmEncryptor::new([0; 32]);
        assert!(result.is_err());
        match result.unwrap_err() {
            CryptoError::InvalidKey(msg) => assert_eq!(msg, "Key must be 32 bytes long"),
            _ => panic!("Expected InvalidKey error"),
        }
        teardown();
    }

    #[test]
    fn test_encrypt_decrypt_success() {
        setup();
        let encryptor = AesGcmEncryptor::new([0; 32]).unwrap();
        let data = b"hello world";
        let encrypted_data = encryptor.encrypt(data).unwrap();
        let decrypted_data = encryptor.decrypt(&encrypted_data).unwrap();
        assert_eq!(data.to_vec(), decrypted_data);
        teardown();
    }

    #[test]
    fn test_decrypt_invalid_data() {
        setup();
        let encryptor = AesGcmEncryptor::new([0; 32]).unwrap();
        let invalid_data = b"invalid data";
        let result = encryptor.decrypt(invalid_data);
        assert!(result.is_err());
        match result.unwrap_err() {
            CryptoError::DecryptionError(msg) => assert_eq!(msg, "Invalid encrypted data"),
            _ => panic!("Expected DecryptionError"),
        }
        teardown();
    }

    #[test]
    fn test_decrypt_wrong_key() {
        setup();
        let encryptor1 = AesGcmEncryptor::new([0; 32]).unwrap();
        let data = b"some secret data";
        let encrypted_data = encryptor1.encrypt(data).unwrap();

        // Create another encryptor with a different key
        unsafe { std::env::set_var("KEYESMERALD", "another_32_byte_key_for_tests") };
        let encryptor2 = AesGcmEncryptor::new([0; 32]).unwrap();

        let result = encryptor2.decrypt(&encrypted_data);
        assert!(result.is_err());
        match result.unwrap_err() {
            CryptoError::DecryptionError(_) => (), // Expected error
            _ => panic!("Expected DecryptionError due to wrong key"),
        }
        teardown();
    }
}
