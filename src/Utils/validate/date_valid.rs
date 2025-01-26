use regex::Regex;

pub fn validate(date: &str) -> bool {
    let date_regex = Regex::new(r"^[0-9]{2,}+/[0-9]{2,}/[0-9]{4,}$").unwrap();
    return date_regex.is_match(date.trim());
}
