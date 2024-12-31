use super::*;

pub fn restore_database(backup_file: &str) -> std::io::Result<()> {
    fs::copy(backup_file, "database.db")?;
    println!("Banco de dados restaurado a partir de: {}", backup_file);
    Ok(())
}
