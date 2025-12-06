use regex::Regex;

pub fn is_alphabetic(string: &str) -> bool {
    for ch in string.chars() {
        if !ch.is_alphabetic() && ch != ' ' {
            return false;
        }
    }
    true
}

pub fn validate_date(date: &str) -> bool {
    let date_regex = Regex::new(r"^[0-9]{2,}+/[0-9]{2,}/[0-9]{4,}$").unwrap();
    date_regex.is_match(date.trim())
}

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email.trim())
}