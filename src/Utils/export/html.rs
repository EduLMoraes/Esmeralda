use super::{mkdir::mkdir, ListCount, Write};

#[allow(dead_code)]
pub async fn export_html(path: &str, data: &ListCount) -> Result<String, String> {
    let (mut file, path) = mkdir(path).await?;

    let mut data_file = String::new();

    let debtors = data.filter_debtors();

    data_file.push_str("<html><head></head><body><table>");
    data_file.push_str("<tr>");
    data_file.push_str(
        "<td>ID_DEVEDOR</td><td>Nome</td><td>Dívida</td><td>Total Gasto</td><td>Status</td>",
    );
    data_file.push_str("</tr>");

    for debtor in debtors {
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
            .trim(),
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
                info.date_in,
                info.date_out,
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
