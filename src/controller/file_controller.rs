use std::env::var;

use crate::{model::{errors::{ControlError, ErrorLog}, list::ListCount}, utils::{export::{csv::export_csv, html::export_html}, import::self_csv::read_csv}};

/// This is to export the datas in a file.
/// Example:
/// ```rs
///     save_in_file("./file.csv", unsafe{ GLOBAL_ACOUNTS.borrorow() });
/// ```
pub async fn save_in_file(path: &str, data: &ListCount) -> Result<String, ControlError> {
    let extend: Vec<&str> = path.split('.').collect();

    let response = match extend[1] {
        "csv" => export_csv(path, data).await,
        // "pdf" => export_pdf(path, data),
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

            tracing::info!("{:?}", err);
            Err(ControlError::ErrorExtern(ErrorLog {
                title: "Error in module export",
                code: 500,
                file: "controller.rs",
            }))
        }
    }
}

pub async fn read_of_file(path: &str, data: &mut ListCount) -> Result<(), ControlError> {
    tracing::info!("Importing file: {}", path);
    let response = read_csv(path, data).await;
    match response {
        Ok(_) => Ok(()),
        Err(err) => {
            let mut path = match std::env::consts::OS {
                "windows" => var("HOMEPATH").unwrap(),
                _ => var("HOME").unwrap(),
            };

            path.push_str("/.esmeralda/log.log");

            tracing::info!("{:?}", err);
            Err(ControlError::ErrorExtern(ErrorLog {
                title: "Error in module export",
                code: 500,
                file: "controller.rs",
            }))
        }
    }
}
