/// Checks if a given string contains only alphabetic characters or spaces.
///
/// # Example
///
/// ```
/// let string1 = String::from("Hello World");
/// let string2 = String::from("Hello123");
///
/// assert_eq!(is_alphabetic(&string1), true);
/// assert_eq!(is_alphabetic(&string2), false);
/// ```
///
/// # Arguments
///
/// * `string` - The input string to be checked.
///
/// # Returns
///
/// Returns `true` if the input string contains only alphabetic characters or spaces, otherwise returns `false`.
pub fn is_alphabetic(string: &String) -> bool {
    for ch in string.chars() {
        if !ch.is_alphabetic() && ch != ' ' {
            return false;
        }
    }

    true
}
