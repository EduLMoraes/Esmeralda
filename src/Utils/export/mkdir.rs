use super::*;

/// Creates a directory structure and returns a file and its path. If the directory already exists, it appends a number to the file name to avoid overwriting existing files.
///
/// # Arguments
///
/// * `path` - A string representing the path to the file that needs to be created.
///
/// # Example
///
/// ```rust
/// let (file, path) = mkdir("path/to/file.txt").await;
/// ```
///
/// # Returns
///
/// A tuple containing the created file and its path.
///
/// # Code Analysis
///
/// The function splits the `path` string into individual directory names and creates the directory structure by iterating over the directory names. It then uses the `create_dir_all` function to create all the directories in the `new_path` string.
///
/// The function determines the limit index where the file name starts by counting the characters until the first dot ('.') is encountered. It initializes variables for the file, the altered path, and a flag to track if the path has been altered.
///
/// The function starts a loop to handle cases where the file already exists. If the file count is between 1 and 10, it appends a number to the file name using the `replace_range` or `insert_str` methods. It prints the altered path for debugging purposes.
///
/// The function checks if the file exists using the `fs::metadata` function. If the file does not exist, it creates it using the `File::create` function and returns the file and the path. If the file already exists, it increments the file count and repeats the loop.
///
/// # Errors
///
/// The function may return an error if there are issues with creating directories or files.
///
/// # Safety
///
/// The function is marked as `async` and may need to be used within an asynchronous context.
#[allow(dead_code)]
pub async fn mkdir(path: &str) -> Result<(File, String), String> {
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for i in 0..paths.len() - 1 {
        new_path.push_str(paths[i]);
        new_path.push('/');
    }

    match create_dir_all(new_path) {
        Ok(_) => {
            let mut limit: usize = 0;
            for a in path.chars() {
                if a == '.' && limit == 0 {
                    limit += 1;
                } else if a == '.' {
                    break;
                } else {
                    limit += 1;
                }
            }

            let mut path = path.to_string();
            let mut is_alterated: bool = false;
            let mut count_files = 0;

            loop {
                if count_files > 0 && count_files < 11 {
                    if is_alterated {
                        path.replace_range(limit..limit + 3, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                } else if count_files > 0 {
                    if is_alterated {
                        path.replace_range(limit..limit + 4, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                }

                match fs::metadata(path.clone()) {
                    Ok(_) => count_files += 1,
                    Err(_) => {
                        let file = match File::create(path.clone()) {
                            Ok(f) => f,
                            Err(e) => return Err(e.to_string()),
                        };

                        return Ok((file, path));
                    }
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
