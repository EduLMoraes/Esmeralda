use std::fs;

 

pub fn backup_database(path: &str) -> Result<(), String> {
    let backup_filename = format!("{}/backup.db", path);

    // Fazendo uma cópia do arquivo de banco de dados (assumindo que seja um SQLite)
    fs::copy("database.db", backup_filename.clone()).map_err(|e|format!("{e:?}"))?;

    tracing::error!("Backup realizado em: {}", backup_filename);
    Ok(())
}
