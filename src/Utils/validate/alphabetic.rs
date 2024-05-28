pub fn is_alphabetic(string: &String) -> bool {
    for ch in string.chars() {
        if !ch.is_alphabetic() && ch != ' ' {
            return false;
        }
    }

    true
}
