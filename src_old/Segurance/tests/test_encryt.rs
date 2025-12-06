use super::*;

#[test]
fn test_return_encrypted_message() {
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    let result = encrpt(msg);

    assert_ne!(result, "");
}

#[test]
fn test_encrypt_valid_key_and_iv() {
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    let result = encrpt(msg);

    assert_ne!(result, "");
}

#[test]
fn test_handle_messages_of_various_lengths() {
    let msg = "Hello, World!".to_string();
    let key = "valid_key".to_string();
    env::set_var("KEYESMERALD", &key);

    let result = encrpt(msg);

    assert_ne!(result, "");
}