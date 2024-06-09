#[allow(unused_imports)]
use super::validate::alphabetic::is_alphabetic;

#[test]
fn test_string_only_alphabetic() {
    let string = String::from("HelloWorld");
    assert!(is_alphabetic(&string));
}

#[test]
fn test_string_only_spaces() {
    let string = String::from("     ");
    assert!(is_alphabetic(&string));
}

#[test]
fn test_string_alphabetic_and_spaces() {
    let string = String::from("Hello World");
    assert!(is_alphabetic(&string));
}

#[test]
fn test_string_non_ascii_characters() {
    let string = String::from("Hello World! 你好");
    assert!(!is_alphabetic(&string));
}

#[test]
fn test_string_non_printable_characters() {
    let string = String::from("Hello\nWorld");
    assert!(!is_alphabetic(&string));
}

#[test]
fn test_string_single_space() {
    let string = String::from(" ");
    assert!(is_alphabetic(&string));
}
