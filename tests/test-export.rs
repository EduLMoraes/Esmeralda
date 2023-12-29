use chrono::NaiveDate;
#[path = "../src/prelude.rs"]
mod prelude;
use prelude::export;
use prelude::structs;
use std::env;

// exports data to a csv file with correct headers and values
#[tokio::test]
async fn test_export_csv_with_correct_headers_and_values() {
    // Arrange
    let temp_dir = env::temp_dir();
    let mut path = temp_dir.as_path().join("test.csv");
    let data = export::InterfaceInfo {
        list: vec![
            structs::Info {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2022-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 3,
                value: 100.0,
                status: true,
            },
            structs::Info {
                id: 2,
                debtor: "Jane Smith".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for goods".to_string(),
                date_in: "2022-02-01".parse::<NaiveDate>().unwrap(),
                date_out: "2022-02-28".parse::<NaiveDate>().unwrap(),
                paid_installments: 2,
                installments: 5,
                value: 200.0,
                status: false,
            },
        ],
    };

    // Act
    let result = export::export_csv(path.to_str().unwrap(), &data).await;

    // Assert
    assert!(result.is_ok());
    path = result.unwrap().into();
    let file_content = std::fs::read_to_string(path).unwrap();
    let expected_content = "ID,Nome,Titulo,Descricao,Data Inicial,Data Final,Parcelas Pagas,Parcelas,Valor,Status\n1,John Doe,Invoice,Payment for services,2022-01-01,2022-01-31,1,3,100.00,true\n2,Jane Smith,Invoice,Payment for goods,2022-02-01,2022-02-28,2,5,200.00,false\n";
    assert_eq!(file_content, expected_content);
}

// creates a new file if the specified path does not exist
#[tokio::test]
async fn test_export_csv_creates_new_file_if_path_does_not_exist() {
    // Arrange
    let temp_dir = env::temp_dir();
    let path = temp_dir.as_path().join("test.csv");
    let data = export::InterfaceInfo {
        list: vec![structs::Info {
            id: 1,
            debtor: "John Doe".to_string(),
            title: "Invoice".to_string(),
            description: "Payment for services".to_string(),
            date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
            date_out: "2022-02-28".parse::<NaiveDate>().unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
        }],
    };

    // Act
    let result = export::export_csv(path.to_str().unwrap(), &data).await;

    // Assert
    assert!(result.is_ok());
    assert!(path.exists());
}

// appends a number to the file name if a file with the same name already exists
#[tokio::test]
async fn test_export_csv_appends_number_to_file_name_if_file_exists() {
    // Arrange
    let temp_dir = env::temp_dir();
    let path = temp_dir.as_path().join("test.csv");
    let existing_file_path = temp_dir.as_path().join("test(1).csv");
    std::fs::write(&existing_file_path, "").unwrap();
    let data = export::InterfaceInfo {
        list: vec![structs::Info {
            id: 1,
            debtor: "John Doe".to_string(),
            title: "Invoice".to_string(),
            description: "Payment for services".to_string(),
            date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
            date_out: "2022-02-28".parse::<NaiveDate>().unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
        }],
    };

    // Act
    let result = export::export_csv(path.to_str().unwrap(), &data).await;

    // Assert
    assert!(result.is_ok());
    assert!(existing_file_path.exists());
    assert!(path.exists());
}

// returns an error if unable to create a new file
#[tokio::test]
async fn test_export_csv_returns_error_if_unable_to_create_file() {
    // Arrange
    let data = export::InterfaceInfo {
        list: vec![structs::Info {
            id: 1,
            debtor: "John Doe".to_string(),
            title: "Invoice".to_string(),
            description: "Payment for services".to_string(),
            date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
            date_out: "2022-02-28".parse::<NaiveDate>().unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
        }],
    };

    // Act
    let result = export::export_csv("/root/test.csv", &data).await;

    // Assert
    assert!(result.is_err());
}

// handles file names with multiple dots correctly
#[tokio::test]
async fn test_export_csv_handles_file_names_with_multiple_dots_correctly() {
    // Arrange
    let temp_dir = env::temp_dir();
    let mut path = temp_dir.as_path().join("test.file.csv");
    let data = export::InterfaceInfo {
        list: vec![structs::Info {
            id: 1,
            debtor: "John Doe".to_string(),
            title: "Invoice".to_string(),
            description: "Payment for services".to_string(),
            date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
            date_out: "2022-01-31".parse::<NaiveDate>().unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
        }],
    };

    // Act
    let result = export::export_csv(path.to_str().unwrap(), &data).await;

    // Assert
    assert!(result.is_ok());

    path = result.unwrap().into();
    let file_content = std::fs::read_to_string(path).unwrap();
    let expected_content = "ID,Nome,Titulo,Descricao,Data Inicial,Data Final,Parcelas Pagas,Parcelas,Valor,Status\n\
                            1,John Doe,Invoice,Payment for services,2022-01-01,2022-01-31,1,3,100.00,true\n";
    assert_eq!(file_content, expected_content);
}
