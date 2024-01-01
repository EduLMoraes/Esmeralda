#[path = "../src/prelude.rs"]
mod prelude;
use prelude::control;
use prelude::structs::*;

use chrono::NaiveDate;

use std::env::temp_dir;
use std::fs;

#[cfg(test)]
#[allow(unused_imports)]
mod tests_exports {
    pub use super::*;

    #[tokio::test]
    async fn test_export_csv_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test.csv");
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        let result = control::save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), path.to_str().unwrap());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_pdf_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test.pdf");
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        let result = control::save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_html_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test.html");
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        let result = control::save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), path.to_str().unwrap());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_non_existent_directory() {
        let path = "/path/to/non_existent_directory/test.csv";
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        let result = control::save_in_file(path, &data).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_read_only_directory() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test.csv");
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        match fs::create_dir_all(path.clone()) {
            Ok(_) => {
                fs::set_permissions(
                    path.to_str().unwrap(),
                    std::os::unix::prelude::PermissionsExt::from_mode(0o444),
                )
                .unwrap();
            }
            Err(_) => {
                fs::set_permissions(
                    path.to_str().unwrap(),
                    std::os::unix::prelude::PermissionsExt::from_mode(0o444),
                )
                .unwrap();
            }
        }

        let result = control::save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_non_existent_file() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("non_existent_directory/test.csv");
        let data = InterfaceInfo {
            list: vec![Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2021-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2021-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            }],
        };

        let result = control::save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }
}
