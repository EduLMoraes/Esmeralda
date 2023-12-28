#[path = "../src/prelude.rs"]
mod prelude;
use prelude::alphabetic;

// should return true for a string containing only alphabetic characters
#[test]
fn test_string_only_alphabetic() {
    let string = String::from("HelloWorld");
    assert_eq!(alphabetic::is_alphabetic(&string), true);
}

    // should return true for a string containing only spaces
#[test]
fn test_string_only_spaces() {
    let string = String::from("     ");
    assert_eq!(alphabetic::is_alphabetic(&string), true);
}

    // should return true for a string containing both alphabetic characters and spaces
#[test]
fn test_string_alphabetic_and_spaces() {
    let string = String::from("Hello World");
    assert_eq!(alphabetic::is_alphabetic(&string), true);
}

    // should return false for a string containing non-ASCII characters
#[test]
fn test_string_non_ascii_characters() {
    let string = String::from("Hello World! 你好");
    assert_eq!(alphabetic::is_alphabetic(&string), false);
}

    // should return false for a string containing non-printable characters
#[test]
fn test_string_non_printable_characters() {
    let string = String::from("Hello\nWorld");
    assert_eq!(alphabetic::is_alphabetic(&string), false);
}

    // should return true for a string containing a single space
#[test]
fn test_string_single_space() {
    let string = String::from(" ");
    assert_eq!(alphabetic::is_alphabetic(&string), true);
}