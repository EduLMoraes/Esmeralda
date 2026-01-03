use super::{FileExportError, FileExporter};
use esmeralda_entities::debt::Debt;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub struct Exporter;

impl Exporter {
    pub fn new() -> Self {
        Self
    }
}

impl FileExporter for Exporter {
    fn export_csv(&self, path: &str, data: &[Debt]) -> Result<String, FileExportError> {
        let path_with_filename = format!("{}/export.csv", path);
        create_dir_all(path).map_err(|e| FileExportError::CreateDir(e.to_string()))?;
        let mut file = File::create(&path_with_filename)
            .map_err(|e| FileExportError::WriteFile(e.to_string()))?;

        let mut data_file = String::new();

        data_file.push_str(
        "Debtor;Nature;Title;Description;Date Start;Date End;Paid Installments;Installments;Value;Status\n",
    );

        for info in data {
            data_file.push_str(
                format!(
                    "{};{};{};{};{};{};{};{};{:.2};{}",
                    info.debtor.name,
                    format!("{:?}", info.nature),
                    info.title,
                    info.description,
                    info.date_start,
                    info.date_end,
                    info.paid_installments,
                    info.installments,
                    info.value,
                    info.status
                )
                .trim(),
            );

            data_file.push('\n');
        }

        file.write_all(data_file.as_bytes())
            .map_err(|e| FileExportError::WriteFile(e.to_string()))?;

        Ok(path_with_filename)
    }

    fn export_html(&self, path: &str, data: &[Debt]) -> Result<String, FileExportError> {
        let path_with_filename = format!("{}/export.html", path);
        create_dir_all(path).map_err(|e| FileExportError::CreateDir(e.to_string()))?;
        let mut file = File::create(&path_with_filename)
            .map_err(|e| FileExportError::WriteFile(e.to_string()))?;

        let mut data_file = String::new();

        data_file.push_str("<html><head></head><body><table>");
        data_file.push_str("<tr>");
        data_file.push_str("<td>Debtor</td><td>Nature</td><td>Title</td><td>Description</td><td>Date Start</td><td>Date End</td><td>Paid Installments</td><td>Installments</td><td>Value</td><td>Status</td>");
        data_file.push_str("</tr>");

        for info in data {
            data_file.push_str("<tr>");

            data_file.push_str(
                format!(
                    "<td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.2}</td><td>{}</td>",
                    info.debtor.name,
                    format!("{:?}", info.nature),
                    info.title,
                    info.description,
                    info.date_start,
                    info.date_end,
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

        file.write_all(data_file.as_bytes())
            .map_err(|e| FileExportError::WriteFile(e.to_string()))?;

        Ok(path_with_filename)
    }
}
