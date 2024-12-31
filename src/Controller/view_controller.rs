use super::*;

#[deprecated]
#[allow(unused)]
pub fn is_alpha(text: &str) -> bool {
    is_alphabetic(text)
}

pub fn is_email(email: &str) -> bool {
    email_valid::validate(email)
}
