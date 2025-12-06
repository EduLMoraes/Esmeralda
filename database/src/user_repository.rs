use super::{Data, Database, DatabaseError, rusqlite_impl::Db};
use esmeralda_entities::user::User;
use rusqlite::params;
use serde_json;

pub struct UserRepository {
    db: Db,
}

impl UserRepository {
    pub fn new(db: Db) -> Self {
        // Create table if it doesn't exist
        db.conn
            .execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                        id TEXT PRIMARY KEY,
                        data TEXT NOT NULL
                    )",
                    User::TITLE
                ),
                [],
            )
            .unwrap();
        Self { db }
    }
}

impl<'a> Database<'a, User> for UserRepository {
    fn insert(&self, data: User) -> Result<(), DatabaseError> {
        let data_json = serde_json::to_string(&data)
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        self.db
            .conn
            .execute(
                &format!("INSERT INTO {} (id, data) VALUES (?1, ?2)", User::TITLE),
                params![data.id.to_string(), data_json],
            )
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        Ok(())
    }

    fn edit(&self, data: User) -> Result<(), DatabaseError> {
        let data_json = serde_json::to_string(&data)
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        self.db
            .conn
            .execute(
                &format!("UPDATE {} SET data = ?2 WHERE id = ?1", User::TITLE),
                params![data.id.to_string(), data_json],
            )
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        Ok(())
    }

    fn suspend(&self, data: User) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                &format!("DELETE FROM {} WHERE id = ?1", User::TITLE),
                params![data.id.to_string()],
            )
            .map_err(|e| DatabaseError::ErrorOnSuspendData(e.to_string()))?;
        Ok(())
    }

    fn get_data(&self, id: &'a str) -> Result<User, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare(&format!("SELECT data FROM {} WHERE id = ?1", User::TITLE))
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![id])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))? {
            let data: String = row.get(0).map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;
            serde_json::from_str(&data)
                .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase("User not found".to_string()))
        }
    }

    fn get_by_email(&self, email: &'a str) -> Result<User, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare(&format!("SELECT data FROM {} WHERE json_extract(data, '$.email') = ?1", User::TITLE))
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![email])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))? {
            let data: String = row.get(0).map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;
            serde_json::from_str(&data)
                .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase("User not found".to_string()))
        }
    }
}
