use super::*;

/// Saves the provided data in a file at the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that represents the file path where the data should be saved.
/// * `data` - A reference to the `ListInfo` data that needs to be saved in the file.
///
/// # Returns
///
/// Returns a `Result` containing a string with the saved file path on success, or a `ControlError` on failure.
///
/// # Example
///
/// ```rust
/// # use crate::ListInfo;
/// # use crate::ControlError;
/// # use crate::ErrorLog;
/// # async fn export_csv(path: &str, data: &ListInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// # async fn export_pdf(path: &str, data: &ListInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// # async fn export_html(path: &str, data: &ListInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// #
/// # #[derive(Debug)]
/// # struct ListInfo;
/// #
/// # #[derive(Debug)]
/// # enum ControlError {
/// #     ErrorValueInvalid(ErrorLog),
/// #     ErrorExtern(ErrorLog),
/// # }
/// #
/// # #[derive(Debug)]
/// # struct ErrorLog {
/// #     title: &'static str,
/// #     code: u32,
/// #     file: &'static str,
/// # }
/// #
/// # async fn save_in_file(path: &str, data: &ListInfo) -> Result<String, ControlError> {
/// let path = "data.csv";
/// let data = ListInfo;
///
/// let result = save_in_file(path, &data).await;
///
/// match result {
///     Ok(path) => println!("File saved successfully at {}", path),
///     Err(error) => println!("Error saving file: {}", error),
/// }
/// # Ok(String::from(""))
/// # }
/// # fn main() {}
/// ```
pub async fn save_in_file(path: &str, data: &ListInfo) -> Result<String, ControlError> {
    let extend: Vec<&str> = path.split('.').collect();

    let response = match extend[1] {
        "csv" => export_csv(path, data).await,
        "pdf" => export_pdf(path, data),
        "html" => export_html(path, data).await,
        _ => {
            return Err(ControlError::ErrorValueInvalid(ErrorLog {
                title: "Extension invalid",
                code: 305,
                file: "controller.rs",
            }))
        }
    };

    match response {
        Ok(path) => Ok(path),
        Err(err) => {
            let mut path = match std::env::consts::OS {
                "windows" => var("HOMEPATH").unwrap(),
                _ => var("HOME").unwrap(),
            };

            path.push_str("/.esmeralda/log.log");

            let _ = log(path.clone().into(), &format!("[CONTROL] {err:?}\n"));
            Err(ControlError::ErrorExtern(ErrorLog {
                title: "Error in module export",
                code: 500,
                file: "controller.rs",
            }))
        }
    }
}
