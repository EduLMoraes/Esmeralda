use esmeralda_entities::debt::Debt;
use thiserror::Error;

pub mod exporter;
pub use exporter::Exporter;

#[derive(Error, Debug)]
pub enum FileExportError {
    #[error("Failed to create directory: {0}")]
    CreateDir(String),
    #[error("Failed to write to file: {0}")]
    WriteFile(String),
}

pub trait FileExporter {
    fn export_csv(&self, path: &str, data: &[Debt]) -> Result<String, FileExportError>;
    fn export_html(&self, path: &str, data: &[Debt]) -> Result<String, FileExportError>;
}
