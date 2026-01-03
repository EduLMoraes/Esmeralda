use super::{rusqlite_impl::Db, Database, DatabaseError};
use esmeralda_entities::user::User;
use rusqlite::params;
use chrono::NaiveDate;

pub struct UserRepository {
    db: Db,
}

impl UserRepository {
    pub fn new(db: Db) -> Self {
        // Create table if it doesn't exist
        db.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Users (
                    id TEXT PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    email TEXT NOT NULL UNIQUE,
                    password TEXT NOT NULL,
                    last_login TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
        Self { db }
    }
}

impl Database<User> for UserRepository {
    fn insert(&self, data: User) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "INSERT INTO Users (id, username, email, password, last_login) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    data.id.to_string(),
                    data.username,
                    data.email,
                    data.password,
                    data.last_login.format("%d-%m-%Y").to_string()
                ],
            )
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        Ok(())
    }

    fn edit(&self, data: User) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "UPDATE Users SET username = ?2, email = ?3, password = ?4, last_login = ?5 WHERE id = ?1",
                params![
                    data.id.to_string(),
                    data.username,
                    data.email,
                    data.password,
                    data.last_login.format("%d-%m-%Y").to_string()
                ],
            )
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        Ok(())
    }

    fn suspend(&self, data: User) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "DELETE FROM Users WHERE id = ?1",
                params![data.id.to_string()],
            )
            .map_err(|e| DatabaseError::ErrorOnSuspendData(e.to_string()))?;
        Ok(())
    }

    fn get_data(&self, id: &str) -> Result<User, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT id, username, email, password, last_login FROM Users WHERE id = ?1")
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![id])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?
        {
            let id: String = row.get(0).unwrap();
            let last_login_str: String = row.get(4).unwrap();
            Ok(User {
                id: id.parse().unwrap(),
                username: row.get(1).unwrap(),
                email: row.get(2).unwrap(),
                password: row.get(3).unwrap(),
                last_login: NaiveDate::parse_from_str(&last_login_str, "%d-%m-%Y").unwrap(),
            })
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase(
                "User not found".to_string(),
            ))
        }
    }

    fn get_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT id, username, email, password, last_login FROM Users WHERE email = ?1")
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![email])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?
        {
            let id: String = row.get(0).unwrap();
            let last_login_str: String = row.get(4).unwrap();
            Ok(User {
                id: id.parse().unwrap(),
                username: row.get(1).unwrap(),
                email: row.get(2).unwrap(),
                password: row.get(3).unwrap(),
                last_login: NaiveDate::parse_from_str(&last_login_str, "%d-%m-%Y").unwrap(),
            })
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase(
                "User not found".to_string(),
            ))
        }
    }
}