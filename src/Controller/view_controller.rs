use super::*;

pub fn is_alpha(text: &String) -> bool {
    is_alphabetic(text)
}

pub fn is_email(email: &String) -> bool {
    email_valid::validate(email)
}
