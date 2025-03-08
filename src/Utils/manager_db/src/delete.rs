use super::*;

pub fn delete_database() -> std::io::Result<()> {
    if fs::remove_file("database.db").is_ok() {
        tracing::error!("Banco de dados deletado.");
    } else {
        etracing::error!("Não foi possível deletar o banco de dados. Ele pode não existir.");
    }
    Ok(())
}
