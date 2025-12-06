#[allow(unused_imports)]
use super::gen_string;

#[test]
fn test_generate_key_length() {
    let key = gen_string(32, &[33, 126]);
    assert_eq!(key.len(), 32);
}

#[test]
fn test_generate_key_random() {
    let key1 = gen_string(32, &[33, 126]);
    let key2 = gen_string(32, &[33, 126]);
    assert_ne!(key1, key2);
}

#[test]
fn test_generate_key_printable_ascii() {
    let key = gen_string(32, &[33, 126]);
    assert!(key.chars().all(|c| c.is_ascii_graphic()));
}

#[test]
fn test_generate_key_unique() {
    let key1 = gen_string(32, &[33, 126]);
    let key2 = gen_string(32, &[33, 126]);
    assert_ne!(key1, key2);
}

#[test]
fn test_generate_key_unpredictable() {
    let key1 = gen_string(32, &[33, 126]);
    let key2 = gen_string(32, &[33, 126]);
    assert_ne!(key1, key2);
}
