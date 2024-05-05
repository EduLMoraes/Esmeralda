#[allow(unused_imports)]
use super::gen_string;

#[test]
fn test_generate_password_length() {
    let password = gen_string(8, &[65, 122]);
    let password2 = gen_string(10, &[65, 122]);
    let password3 = gen_string(-7, &[65, 122]);
    assert_eq!(password.len(), 8);
    assert_eq!(password2.len(), 10);
    assert!(password3.is_empty());
    let password3 = gen_string(0, &[65, 122]);
    assert!(password3.is_empty());
}

#[test]
fn test_generate_password_random() {
    let password1 = gen_string(8, &[33, 126]);
    let password2 = gen_string(8, &[33, 126]);
    assert_ne!(password1, password2);
}

#[test]
fn test_generate_password_printable_ascii() {
    let password = gen_string(8, &[33, 126]);
    assert!(password.chars().all(|c| c.is_ascii_graphic()));
}

#[test]
fn test_generate_password_unique() {
    let password1 = gen_string(8, &[33, 126]);
    let password2 = gen_string(8, &[33, 126]);
    assert_ne!(password1, password2);
}

#[test]
fn test_generate_password_unpredictable() {
    let password1 = gen_string(8, &[33, 126]);
    let password2 = gen_string(8, &[33, 126]);
    assert_ne!(password1, password2);
}
