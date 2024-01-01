#[path = "../src/prelude.rs"]
mod prelude;
use prelude::email_valid;

#[test]
fn should_return_true_for_valid_email_address() {
    let email = String::from("test@example.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, true);
}

#[test]
fn should_return_false_for_invalid_email_address() {
    let email = String::from("invalid_email");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

#[test]
fn should_return_false_for_empty_email_address() {
    let email = String::from("");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

#[test]
fn should_return_false_for_email_address_without_top_level_domain() {
    let email = String::from("test@example");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

#[test]
fn should_return_false_for_email_address_without_domain() {
    let email = String::from("test@.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

#[test]
fn should_return_false_for_email_address_without_username() {
    let email = String::from("@example.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}
