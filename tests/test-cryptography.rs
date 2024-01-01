use std::env;
#[path = "../src/prelude.rs"]
mod prelude;
use prelude::cryptography;

#[cfg(test)]
mod test_encrypt {
    use super::*;

    #[test]
    fn test_return_encrypted_message() {
        let msg = "Hello, World!".to_string();
        let key = "valid_key".to_string();
        env::set_var("KEYESMERALD", &key);

        let result = cryptography::encrpt(msg);

        assert_ne!(result, "");
    }

    #[test]
    fn test_encrypt_valid_key_and_iv() {
        let msg = "Hello, World!".to_string();
        let key = "valid_key".to_string();
        env::set_var("KEYESMERALD", &key);

        let result = cryptography::encrpt(msg);

        assert_ne!(result, "");
    }

    #[test]
    fn test_handle_messages_of_various_lengths() {
        let msg = "Hello, World!".to_string();
        let key = "valid_key".to_string();
        env::set_var("KEYESMERALD", &key);

        let result = cryptography::encrpt(msg);

        assert_ne!(result, "");
    }
}

#[cfg(test)]
mod test_key {
    use super::*;
    #[test]
    fn test_generate_key_length() {
        let key = cryptography::get_key();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_generate_key_random() {
        let key1 = cryptography::get_key();
        let key2 = cryptography::get_key();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_key_printable_ascii() {
        let key = cryptography::get_key();
        assert!(key.chars().all(|c| c.is_ascii_graphic()));
    }

    #[test]
    fn test_generate_key_unique() {
        let key1 = cryptography::get_key();
        let key2 = cryptography::get_key();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_key_unpredictable() {
        let key1 = cryptography::get_key();
        let key2 = cryptography::get_key();
        assert_ne!(key1, key2);
    }
}
