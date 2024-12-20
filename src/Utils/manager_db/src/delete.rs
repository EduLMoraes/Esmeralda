use super::*;

pub fn delete_database() -> std::io::Result<()> {
    if fs::remove_file("database.db").is_ok() {
        println!("Banco de dados deletado.");
    } else {
        eprintln!("Não foi possível deletar o banco de dados. Ele pode não existir.");
    }
    Ok(())
}
