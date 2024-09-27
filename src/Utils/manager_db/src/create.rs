use std::os::unix::fs::PermissionsExt;

use super::*;

pub fn create_database(conn: &Connection, last_version: bool) -> Result<()> {
    if last_version {
        println!("Criando banco de dados na última versão...");
    } else {
        println!("Criando banco de dados com versão especificada...");
    }

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id      INTEGER PRIMARY KEY,
                  name    TEXT NOT NULL,
                  email   TEXT NOT NULL UNIQUE
                  )",
        [],
    )?;

    use fs::Permissions;
    let permissions = Permissions::from_mode(0o000);
    fs::set_permissions(PATH, permissions).unwrap();

    Ok(())
}
