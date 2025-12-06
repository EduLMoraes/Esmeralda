use rusqlite::Connection;
use std::sync::Arc;

pub struct Db {
    pub conn: Arc<Connection>,
}

impl Db {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Db {
            conn: Arc::new(conn),
        })
    }

    pub fn new_with_path(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        Ok(Db {
            conn: Arc::new(conn),
        })
    }
}
