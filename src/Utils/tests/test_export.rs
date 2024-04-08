#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::prelude::env;

#[allow(unused_imports)]
use crate::prelude::env::temp_dir;

#[allow(unused_imports)]
use std::path::Path;

#[allow(unused_imports)]
use chrono::NaiveDate;

#[cfg(test)]
mod test_export_csv {
    use super::*;
    // #[tokio::test]
    async fn test_export_csv_with_correct_headers_and_values() {
        let temp_dir = env::temp_dir();
        let mut path = temp_dir.as_path().join("test.csv");
        let data = ListCount {
            list: vec![
                Count {
                    id: 1,
                    debtor: "John Doe".to_string(),
                    title: "Invoice".to_string(),
                    description: "Payment for services".to_string(),
                    date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
                    date_out: "2022-01-31".parse::<NaiveDate>().unwrap(),
                    paid_installments: 1,
                    installments: 1,
                    value: 100.0,
                    status: true,
                    nature: String::from("Investimentos"),
                },
                Count {
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
                    nature: String::from("Investimentos"),
                },
            ],
            years: vec![],
        };

        let result = export::csv::export_csv(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        path = result.unwrap().into();
        let file_content = std::fs::read_to_string(path).unwrap();
        let expected_content = "ID_DEVEDOR;Devedor;Dívida;Total Gasto;Status\n\
                                1;John Doe;0.00;100.00;true\n\
                                2;Jane Smith;600.00;400.00;false\n\
                                \nID_CONTA;Nome;Natureza do Gasto;Titulo;Descricao;Data Inicial;Data Final;Parcelas Pagas;Parcelas;Valor;Status\n\
                                1;John Doe;Investimentos;Invoice;Payment for services;2022-01-01;2022-01-31;1;1;100.00;true\n\
                                2;Jane Smith;Investimentos;Invoice;Payment for goods;2022-02-01;2022-02-28;2;5;200.00;false\n";
        assert_eq!(file_content, expected_content);
    }

    // #[tokio::test]
    async fn test_export_csv_creates_new_file_if_path_does_not_exist() {
        let temp_dir = env::temp_dir();
        let path = temp_dir.as_path().join("test.csv");
        let data = ListCount {
            list: vec![Count {
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
                nature: String::from("Investimentos"),
            }],
            years: vec![],
        };

        let result = export::csv::export_csv(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(path.exists());
    }
    // #[tokio::test]
    async fn test_export_csv_appends_number_to_file_name_if_file_exists() {
        let temp_dir = env::temp_dir();
        let path = temp_dir.as_path().join("test.csv");
        let existing_file_path = temp_dir.as_path().join("test(1).csv");
        std::fs::write(&existing_file_path, "").unwrap();
        let data = ListCount {
            list: vec![Count {
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
                nature: String::from("Investimentos"),
            }],
            years: vec![],
        };

        let result = export::csv::export_csv(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(existing_file_path.exists());
        assert!(path.exists());
    }

    // #[tokio::test]
    async fn test_export_csv_returns_error_if_unable_to_create_file() {
        let data = ListCount {
            list: vec![Count {
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
                nature: String::from("Investimentos"),
            }],
            years: vec![],
        };

        let result = export::csv::export_csv("/root/test.csv", &data).await;

        assert!(result.is_err());
    }

    // #[tokio::test]
    async fn test_export_csv_handles_file_names_with_multiple_dots_correctly() {
        let temp_dir = env::temp_dir();
        let mut path = temp_dir.as_path().join("test.file.csv");
        let data = ListCount {
            list: vec![Count {
                id: 1,
                debtor: "John Doe".to_string(),
                title: "Invoice".to_string(),
                description: "Payment for services".to_string(),
                date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
                date_out: "2022-01-31".parse::<NaiveDate>().unwrap(),
                paid_installments: 1,
                installments: 1,
                value: 100.0,
                status: true,
                nature: String::from("Investimentos"),
            }],
            years: vec![],
        };

        let result = export::csv::export_csv(path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());

        path = result.unwrap().into();
        let file_content = std::fs::read_to_string(path).unwrap();
        let expected_content = "ID_DEVEDOR;Devedor;Dívida;Total Gasto;Status\n\
                                1;John Doe;0.00;100.00;true\n\
                                \nID_CONTA;Nome;Natureza do Gasto;Titulo;Descricao;Data Inicial;Data Final;Parcelas Pagas;Parcelas;Valor;Status\n\
                                1;John Doe;Investimentos;Invoice;Payment for services;2022-01-01;2022-01-31;1;1;100.00;true\n";
        assert_eq!(file_content, expected_content);
    }
}

#[cfg(test)]
mod test_export_html {
    use super::*;

    // #[tokio::test]
    async fn test_export_html_valid_input() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.as_path().join("output.html");
        let data = ListCount::new();

        let result = export::html::export_html(file_path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(Path::new(file_path.to_str().unwrap()).exists());
    }

    // #[tokio::test]
    async fn test_export_html_empty_input() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.as_path().join("output.html");
        let data = ListCount {
            list: vec![],
            years: vec![],
        };

        let result = export::html::export_html(file_path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(Path::new(file_path.to_str().unwrap()).exists());
    }

    // #[tokio::test]
    async fn test_export_html_special_characters_input() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.as_path().join("output.html");
        let data = ListCount::new();

        let result = export::html::export_html(file_path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
        assert!(Path::new(file_path.to_str().unwrap()).exists());
    }

    // #[tokio::test]
    async fn test_export_html_unable_to_create_directory() {
        let file_path = "/nonexistent_directory/output.html";
        let data = ListCount::new();

        let result = export::html::export_html(file_path, &data).await;

        assert!(result.is_err());
    }

    // #[tokio::test]
    async fn test_export_html_create_file_not_exists() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.as_path().join("nonexistent_directory/output.html");
        let data = ListCount::new();

        let result = export::html::export_html(file_path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }

    // #[tokio::test]
    async fn test_export_html_write_to_file() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.as_path().join("output.html");
        let data = ListCount::new();

        let result = export::html::export_html(file_path.to_str().unwrap(), &data).await;

        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_export_pdf {
    use super::*;

    #[test]
    fn test_export_pdf_with_expected_format_and_content() {
        let path = "test.pdf";
        let data = ListCount::new();

        let result = export::pdf::export_pdf(path, &data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_export_pdf_with_empty_input_data() {
        let path = "test.pdf";
        let data = ListCount::new();

        let result = export::pdf::export_pdf(path, &data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_export_pdf_with_very_large_input_data() {
        let path = "test.pdf";
        let mut data = ListCount::new();

        for _ in 0..10000 {
            data.put(Count {
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
                nature: String::from("Casa"),
            });
        }

        let result = export::pdf::export_pdf(path, &data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_export_pdf_with_very_small_input_data() {
        let path = "test.pdf";
        let data = ListCount::new();

        let result = export::pdf::export_pdf(path, &data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_export_pdf_with_very_long_strings_in_input_data() {
        let path = "test.pdf";
        let mut data = ListCount::new();

        data.put( Count{
            id: 1,
            debtor: "John Doe Silver Algostin Guilherme Santos Pedro Fabiancio".to_string(),
            title: "Invoice the RPG in game of life on real life, like video of youtube with montage of dragons and dungeons".to_string(),
            description: "Payment for services in multiplataform of game of thrones and this is so test with long strings.".to_string(),
            date_in: "2022-01-01".parse::<NaiveDate>().unwrap(),
            date_out: "2022-02-28".parse::<NaiveDate>().unwrap(),
            paid_installments: 1,
            installments: 3,
            value: 100.0,
            status: true,
            nature: String::from("Alimentação")
        } );

        let result = export::pdf::export_pdf(path, &data);

        assert!(result.is_ok());
    }
}
