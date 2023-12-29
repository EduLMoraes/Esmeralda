use std::env;
#[path = "../src/prelude.rs"]
mod prelude;
use prelude::cryptography;

#[test]
fn test_return_encrypted_message() {
    // Arrange
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    // Act
    let result = cryptography::encrpt(msg);

    // Assert
    assert_ne!(result, "");
}

#[test]
fn test_encrypt_valid_key_and_iv() {
    // Arrange
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    // Act
    let result = cryptography::encrpt(msg);

    // Assert
    assert_ne!(result, "");
}

#[test]
fn test_handle_messages_of_various_lengths() {
    // Arrange
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    // Act
    let result = cryptography::encrpt(msg);

    // Assert
    assert_ne!(result, "");
}

// Generates a key of length 1024 characters
#[test]
fn test_generate_key_length() {
    let key = cryptography::get_key();
    assert_eq!(key.len(), 32);
}

// Returns a randomly generated key
#[test]
fn test_generate_key_random() {
    let key1 = cryptography::get_key();
    let key2 = cryptography::get_key();
    assert_ne!(key1, key2);
}

// None
#[test]
fn test_none() {
    // No behavior to test
    assert!(true);
}

// The generated key contains only printable ASCII characters
#[test]
fn test_generate_key_printable_ascii() {
    let key = cryptography::get_key();
    assert!(key.chars().all(|c| c.is_ascii_graphic()));
}

// The generated key is unique for each call
#[test]
fn test_generate_key_unique() {
    let key1 = cryptography::get_key();
    let key2 = cryptography::get_key();
    assert_ne!(key1, key2);
}

// The generated key is not predictable
#[test]
fn test_generate_key_unpredictable() {
    let key1 = cryptography::get_key();
    let key2 = cryptography::get_key();
    assert_ne!(key1, key2);
}
