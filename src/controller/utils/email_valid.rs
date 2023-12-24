use regex::Regex;

pub fn validate(email: &String) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if email_regex.is_match(email.trim()) {
        return true;
    } else {
        return false;
    }
}