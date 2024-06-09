use super::{mkdir::mkdir, ListCount, Write};

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
                info.date_in,
                info.date_out,
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
