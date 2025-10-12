use std::fs;

pub fn restore_database(backup_file: &str) -> Result<(), String> {
    fs::copy(backup_file, "database.db").map_err(|e| format!("{e:?}"))?;
    tracing::error!("Banco de dados restaurado a partir de: {}", backup_file);
    Ok(())
}
