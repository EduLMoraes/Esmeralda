#[path = "../src/prelude.rs"]
mod prelude;
use prelude::email_valid;

// Should return true for a valid email address
#[test]
fn should_return_true_for_valid_email_address() {
    let email = String::from("test@example.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, true);
}

    // Should return false for an invalid email address
#[test]
fn should_return_false_for_invalid_email_address() {
    let email = String::from("invalid_email");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

    // Should return false for an empty email address
#[test]
fn should_return_false_for_empty_email_address() {
    let email = String::from("");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

    // Should return false for an email address without a top-level domain
#[test]
fn should_return_false_for_email_address_without_top_level_domain() {
    let email = String::from("test@example");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

    // Should return false for an email address without a domain
#[test]
fn should_return_false_for_email_address_without_domain() {
    let email = String::from("test@.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}

    // Should return false for an email address without a username
#[test]
fn should_return_false_for_email_address_without_username() {
    let email = String::from("@example.com");
    let is_valid = email_valid::validate(&email);
    assert_eq!(is_valid, false);
}
