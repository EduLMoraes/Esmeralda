use super::*;

pub fn is_alpha(text: &String) -> bool {
    is_alphabetic(text)
}

pub fn is_email(email: &String) -> bool {
    email_valid::validate(email)
}

pub fn signed_month_difference(start: NaiveDate, end: NaiveDate) -> i32 {
    let end_naive = end;
    let start_naive = start;

    let month_diff = end_naive.month() as i32 - start_naive.month() as i32;
    let years_diff = (end_naive.year() - start_naive.year()) as i32;

    if month_diff > 0 {
        (years_diff * 12) + month_diff
    } else {
        (years_diff - 1) * 12 + (month_diff + 12)
    }
}

/// Checks if the given `Count` struct contains all the required information for it to be considered complete.
///
/// # Example
///
/// ```
/// use futures::executor::block_on;
///
/// #[derive(Debug)]
/// struct Count {
///     debtor: String,
///     title: String,
///     value: f64,
///     installments: u32,
/// }
///
/// async fn is_complete(info: &Count) -> bool {
///     if info.debtor.is_empty() || !is_alphabetic(&info.debtor) {
///         return false;
///     } else if info.title.is_empty() {
///         return false;
///     } else if info.value == 0.0 {
///         return false;
///     } else if info.installments == 0 {
///         return false;
///     }
///
///     true
/// }
///
/// fn is_alphabetic(s: &str) -> bool {
///     s.chars().all(|c| c.is_alphabetic() || c == ' ')
/// }
///
/// fn main() {
///     let info = Count {
///         debtor: String::from("John Doe"),
///         title: String::from("Payment"),
///         value: 100.0,
///         installments: 2,
///     };
///
///     let result = block_on(is_complete(&info));
///     println!("Is the info complete? {}", result);
/// }
/// ```
pub async fn is_complete(info: &Count) -> bool {
    if info.debtor.trim().is_empty() || !is_alphabetic(&info.debtor) {
        return false;
    } else if info.title.is_empty() {
        return false;
    } else if info.value == 0.0 {
        return false;
    } else if info.installments == 0 {
        return false;
    }

    true
}
