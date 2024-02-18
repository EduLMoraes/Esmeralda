use rand::{thread_rng, Rng};

/// Generates a random key of length 1024 characters.
///
/// # Example
///
/// ```
/// let key = get_key();
/// ```
///
/// # Returns
///
/// A randomly generated key of length 1024 characters.
#[allow(dead_code)]
pub fn get_key() -> String {
    let mut key = String::new();

    for _ in 0..32 {
        let index: u8 = thread_rng().gen_range(33..126);
        key.push(index as char);
    }

    key
}
