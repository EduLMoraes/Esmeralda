#[allow(unused_assignments)]
/// Updates the initial and ending values based on certain conditions.
///
/// # Arguments
///
/// * `init` - An initial value.
/// * `end` - An ending value.
/// * `lines` - The number of lines.
///
/// # Returns
///
/// A tuple containing the updated initial value (`init`) and the updated ending value (`end`).
pub fn back_page(mut init: usize, mut end: usize, lines: usize) -> (usize, usize) {
    if init > 0 {
        end = init;
        init -= lines;
    } else if init > 0 && init < lines {
        end = init;
        init += init - 0;
    } else {
        init = 0;
        end = lines;
    }

    (init, end)
}

/// This function takes in five parameters: `init`, `end`, `lines`, `size_max`, and returns a tuple of two usize values.
///
/// # Arguments
///
/// * `init` - An initial usize value.
/// * `end` - An ending usize value.
/// * `lines` - The number of lines.
/// * `size_max` - A reference to the maximum size.
///
/// # Example
///
/// ```
/// let init = 0;
/// let end = 10;
/// let lines = 5;
/// let size_max = &20;
///
/// let (new_init, new_end) = next_page(init, end, lines, size_max);
///
/// println!("New init: {}", new_init);
/// println!("New end: {}", new_end);
/// ```
///
/// # Returns
///
/// A tuple containing the updated initial usize value and the updated ending usize value.
pub fn next_page(
    mut init: usize,
    mut end: usize,
    lines: usize,
    size_max: &usize,
) -> (usize, usize) {
    if end <= size_max - lines {
        init = end;
        end += lines;
    } else if end < *size_max && end > size_max - lines {
        init = end;
        end += size_max - end;
    }

    (init, end)
}
