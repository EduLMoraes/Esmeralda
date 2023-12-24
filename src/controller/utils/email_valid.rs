use regex::Regex;

/// Validates an email address.
///
/// This function takes an email address as input and returns a boolean value indicating whether the email address is valid or not.
///
/// # Example
///
/// ```
/// let email = String::from("test@example.com");
/// let is_valid = validate(&email);
/// println!("Is valid email? {}", is_valid);
/// ```
///
/// # Arguments
///
/// * `email` - The email address to be validated.
///
/// # Returns
///
/// Returns `true` if the email address is valid, `false` otherwise.
pub fn validate(email: &String) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if email_regex.is_match(email.trim()) {
        return true;
    } else {
        return false;
    }
}