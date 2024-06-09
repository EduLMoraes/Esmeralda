use super::*;

#[allow(dead_code)]
pub async fn mkdir(path: &str) -> Result<(File, String), String> {
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for item in paths.iter().take(paths.len() - 1) {
        new_path.push_str(item);
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
