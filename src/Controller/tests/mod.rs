#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::prelude::env::temp_dir;

#[allow(unused_imports)]
use std::fs;

#[cfg(test)]
#[allow(unused_imports)]
mod test_save_in_file {
    pub use super::*;

    #[tokio::test]
    async fn test_export_csv_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test-control.csv");
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
                nature: "Casa".to_string(),
            }],
        };

        let result = save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_pdf_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test-control.pdf");
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
                nature: "Casa".to_string(),
            }],
        };

        let result = save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_html_success() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test-control.html");
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
                nature: "Casa".to_string(),
            }],
        };

        let result = save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(fs::metadata(path).is_ok());
    }

    #[tokio::test]
    async fn test_export_non_existent_directory() {
        let path = "/path/to/non_existent_directory/test-control.csv";
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
                nature: "Casa".to_string(),
            }],
        };

        let result = save_in_file(path, &data).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_read_only_directory() {
        let temp_dir = temp_dir();
        let path = temp_dir.as_path().join("test-control.csv");
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
                nature: "Casa".to_string(),
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

        let result = save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_non_existent_file() {
        let temp_dir = temp_dir();
        let path = temp_dir
            .as_path()
            .join("non_existent_directory/test-control.csv");
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
                nature: "Casa".to_string(),
            }],
        };

        let result = save_in_file(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }
}

mod test_is_complete {
    #[allow(unused_imports)]
    pub use super::*;

    // Returns true if all fields in Info struct are non-empty and valid.
    #[tokio::test]
    async fn test_all_fields_non_empty_and_valid() {
        let info = Info {
            id: 1,
            debtor: String::from("John Doe"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
            nature: String::from("Casa"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, true);
    }

    // Returns false if debtor field is empty or contains non-alphabetic characters.
    #[tokio::test]
    async fn test_debtor_field_empty_or_non_alphabetic() {
        let info = Info {
            id: 1,
            debtor: String::from(""),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
            nature: String::from("Casa"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if title field is empty.
    #[tokio::test]
    async fn test_title_field_empty() {
        let info = Info {
            id: 0,
            debtor: String::from("John Doe"),
            title: String::from(""),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            installments: 1,
            value: 100.0,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if value field is zero.
    #[tokio::test]
    async fn test_value_field_zero() {
        let info = Info {
            id: 0,
            debtor: String::from("John Doe"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            installments: 1,
            value: 0.0,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if installments field is zero.
    #[tokio::test]
    async fn test_installments_field_zero() {
        let info = Info {
            id: 0,
            debtor: String::from("John Doe"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            installments: 0,
            value: 100.0,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if debtor field contains only spaces.
    #[tokio::test]
    async fn test_debtor_field_only_spaces() {
        let info = Info {
            id: 1,
            debtor: String::from("     "),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 100.0,
            installments: 2,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if debtor field contains only non-alphabetic characters.
    #[tokio::test]
    async fn test_debtor_field_only_non_alphabetic() {
        let info = Info {
            id: 1,
            debtor: String::from("12345"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 100.0,
            installments: 2,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns true if debtor field contains alphabetic characters and spaces.
    #[tokio::test]
    async fn test_debtor_field_alphabetic_and_spaces() {
        let info = Info {
            id: 1,
            debtor: String::from("John Doe     "),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 100.0,
            installments: 2,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, true);
    }

    // Returns true if value field is non-zero.
    #[tokio::test]
    async fn test_value_field_non_zero() {
        let info = Info {
            id: 1,
            debtor: String::from("John Doe"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 100.0,
            installments: 2,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, true);
    }

    // Returns true if installments field is non-zero.
    #[tokio::test]
    async fn test_installments_field_non_zero() {
        let info = Info {
            id: 1,
            debtor: String::from("John Doe"),
            title: String::from("Invoice"),
            description: String::from("Payment for services"),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 100.0,
            installments: 2,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, true);
    }

    // Returns false if Info struct is empty.
    #[tokio::test]
    async fn test_info_struct_empty() {
        let info = Info {
            id: 0,
            debtor: String::from(""),
            title: String::from(""),
            description: String::from(""),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 0.0,
            installments: 0,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }

    // Returns false if Info struct has only debtor field.
    #[tokio::test]
    async fn test_info_struct_only_debtor_field() {
        let info = Info {
            id: 0,
            debtor: String::from("John Doe"),
            title: String::from(""),
            description: String::from(""),
            date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
            paid_installments: 0,
            value: 0.0,
            installments: 0,
            status: false,
            nature: String::from("Investimentos"),
        };

        let result = is_complete(&info).await;
        assert_eq!(result, false);
    }
}
