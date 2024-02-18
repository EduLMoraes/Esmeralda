use super::{
    InterfaceInfo,
    mkdir::mkdir,
    Write
};

/// Export HTML
///
/// The `export_html` function exports data to an HTML file. It takes a `path` and `data` as inputs and returns a `Result` containing either the path of the exported HTML file or an error message.
///
/// # Example Usage
///
/// ```rust
/// let path = "output.html";
/// let data = InterfaceInfo { ... };
///
/// match export_html(path, &data).await {
///     Ok(file_path) => println!("HTML file exported successfully: {}", file_path),
///     Err(error) => println!("Error exporting HTML file: {}", error),
/// }
/// ```
///
/// # Arguments
///
/// * `path` - A string representing the file path where the HTML file will be exported.
/// * `data` - A reference to an `InterfaceInfo` struct containing the data to be included in the HTML table.
///
/// # Returns
///
/// * If the HTML file is exported successfully, the function returns a `Result` containing the path of the exported file.
/// * If there is an error during the file export, the function returns a `Result` containing an error message.
#[allow(dead_code)]
pub async fn export_html(path: &str, data: &InterfaceInfo) -> Result<String, String> {
    let (mut file, path) = mkdir(path).await?;

    let mut data_file = String::new();

    let debtors = data.filter_debtors();

    data_file.push_str("<html><head></head><body><table>");
    data_file.push_str("<tr>");
    data_file.push_str("<td>ID_DEVEDOR</td><td>Nome</td><td>Dívida</td><td>Total Gasto</td><td>Status</td>");
    data_file.push_str("</tr>");

    for debtor in debtors{
        data_file.push_str("<tr>");

        data_file.push_str(
            format!(
                "<td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>",
                debtor.get_id(),
                debtor.get_name(),
                debtor.get_debt(),
                debtor.get_value(),
                debtor.get_status()
            )
        .trim()
        );

        data_file.push_str("</tr>");
    }

    data_file.push_str("</table>");

    data_file.push_str("<table>");
    data_file.push_str("<tr>");
    data_file.push_str("<td>ID</td><td>Nome</td><td>Natureza do gasto</td><td>Titulo</td><td>Descricao</td><td>Data Inicial</td><td>Data Final</td><td>Parcelas</td><td>Valor</td><td>Status</td>");
    data_file.push_str("</tr>");

    for info in &data.list {
        data_file.push_str("<tr>");

        data_file.push_str(
            format!(
                "<td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}/{}</td><td>{:.2}</td><td>{}</td>",
                info.id,
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

        data_file.push_str("</tr>");
    }

    data_file.push_str("</table></body></html>");

    match file.write_all(data_file.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}
