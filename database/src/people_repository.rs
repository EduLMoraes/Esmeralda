use super::{Data, Database, DatabaseError, rusqlite_impl::Db};
use esmeralda_entities::people::People;
use rusqlite::params;
use serde_json;

pub struct PeopleRepository {
    db: Db,
}

impl PeopleRepository {
    pub fn new(db: Db) -> Self {
        // Create table if it doesn't exist
        db.conn
            .execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                        id TEXT PRIMARY KEY,
                        data TEXT NOT NULL
                    )",
                    People::TITLE
                ),
                [],
            )
            .unwrap();
        Self { db }
    }
}

impl<'a> Database<'a, People> for PeopleRepository {
    fn insert(&self, data: People) -> Result<(), DatabaseError> {
        let data_json = serde_json::to_string(&data)
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        self.db
            .conn
            .execute(
                &format!("INSERT INTO {} (id, data) VALUES (?1, ?2)", People::TITLE),
                params![data.id.to_string(), data_json],
            )
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        Ok(())
    }

    fn edit(&self, data: People) -> Result<(), DatabaseError> {
        let data_json = serde_json::to_string(&data)
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        self.db
            .conn
            .execute(
                &format!("UPDATE {} SET data = ?2 WHERE id = ?1", People::TITLE),
                params![data.id.to_string(), data_json],
            )
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        Ok(())
    }

    fn suspend(&self, data: People) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                &format!("DELETE FROM {} WHERE id = ?1", People::TITLE),
                params![data.id.to_string()],
            )
            .map_err(|e| DatabaseError::ErrorOnSuspendData(e.to_string()))?;
        Ok(())
    }

    fn get_data(&self, id: &'a str) -> Result<People, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare(&format!("SELECT data FROM {} WHERE id = ?1", People::TITLE))
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![id])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))? {
            let data: String = row.get(0).map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;
            serde_json::from_str(&data)
                .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase("People not found".to_string()))
        }
    }
}
