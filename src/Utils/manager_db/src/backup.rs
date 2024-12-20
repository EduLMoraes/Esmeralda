use super::*;

pub fn backup_database(path: &str) -> std::io::Result<()> {
    let backup_filename = format!("{}/backup.db", path);

    // Fazendo uma cópia do arquivo de banco de dados (assumindo que seja um SQLite)
    fs::copy("database.db", backup_filename.clone())?;

    println!("Backup realizado em: {}", backup_filename);
    Ok(())
}
