use super::{mkdir::mkdir, ListCount, Write};

/// # export_csv
///
/// A function that takes a `path` and `data` as inputs and returns a `Result` containing either the path of the created file or an error message.
///
/// ## Example Usage
/// ```rust
/// let result = export_csv("path/to/file.csv", &List_info).await;
/// ```
///
/// ## Arguments
/// - `path` (string): The path to the file that needs to be created.
/// - `data` (ListCount): The data that will be written to the file.
///
/// ## Returns
/// A `Result` object that contains either the path of the created file or an error message.
#[allow(dead_code)]
pub async fn export_csv(path: &str, data: &ListCount) -> Result<String, String> {
    let (mut file, path) = mkdir(path).await?;

    let mut data_file = String::new();

    let debtors = data.filter_debtors();

    data_file.push_str("Devedor;Dívida;Total Gasto;Status\n");

    for debtor in debtors {
        data_file.push_str(
            format!(
                "{};{:.2};{:.2};{}",
                debtor.get_name(),
                debtor.get_debt(),
                debtor.get_value(),
                debtor.get_status()
            )
            .trim(),
        );

        data_file.push('\n');
    }

    data_file.push_str(
        "\nNome;Natureza do Gasto;Titulo;Descricao;Data Inicial;Data Final;Parcelas Pagas;Parcelas;Valor;Status\n",
    );

    for info in &data.list {
        data_file.push_str(
            format!(
                "{};{};{};{};{};{};{};{};{:.2};{}",
                info.debtor,
                info.nature,
                info.title,
                info.description,
                info.date_in.to_string(),
                info.date_out.to_string(),
                info.paid_installments,
                info.installments,
                info.value,
                info.status
            )
            .trim(),
        );

        data_file.push('\n');
    }

    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}
