// use std::os::unix::fs::PermissionsExt;
#[path = "./versions/mod.rs"]
mod versions;
use super::*;

pub fn create_database(conn: &Connection, last_version: String) -> Result<()> {
    let _ = match last_version.trim() {
        "1.2.3" => conn.execute_batch(&versions::v1_2_3::get_sql()),
        _ => conn.execute_batch(&versions::v1_3_0::get_sql()),
    };

    //    use fs::Permissions;
    //  let permissions = Permissions::from_mode(0o600);
    //fs::set_permissions(conn.path().unwrap(), permissions).unwrap();

    Ok(())
}
