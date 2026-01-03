use super::{rusqlite_impl::Db, Database, DatabaseError};
use esmeralda_entities::debt::{Debt, NatureDebt};
use rusqlite::params;
use esmeralda_entities::people::People;
use chrono::NaiveDate;

pub struct DebtRepository {
    db: Db,
}

impl DebtRepository {
    pub fn new(db: Db) -> Self {
        // Create table if it doesn't exist
        db.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Counts (
                    id TEXT PRIMARY KEY,
                    id_user TEXT NOT NULL,
                    paid_installments INTEGER NOT NULL,
                    installments INTEGER NOT NULL,
                    debtor TEXT NOT NULL,
                    value REAL NOT NULL,
                    title TEXT NOT NULL,
                    date_out TEXT NOT NULL,
                    date_in TEXT NOT NULL,
                    proof TEXT,
                    nature TEXT NOT NULL,
                    description TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
        Self { db }
    }
}

impl Database<Debt> for DebtRepository {
    fn insert(&self, data: Debt) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "INSERT INTO Counts (id, id_user, paid_installments, installments, debtor, value, title, date_out, date_in, proof, nature, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    data.id.to_string(),
                    data.debtor.id.to_string(),
                    data.paid_installments,
                    data.installments,
                    data.debtor.id.to_string(),
                    data.value,
                    data.title,
                    data.date_end.format("%d-%m-%Y").to_string(),
                    data.date_start.format("%d-%m-%Y").to_string(),
                    data.proof,
                    format!("{:?}", data.nature),
                    data.description
                ],
            )
            .map_err(|e| DatabaseError::ErrorOnInsertData(e.to_string()))?;
        Ok(())
    }

    fn edit(&self, data: Debt) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "UPDATE Counts SET id_user = ?2, paid_installments = ?3, installments = ?4, debtor = ?5, value = ?6, title = ?7, date_out = ?8, date_in = ?9, proof = ?10, nature = ?11, description = ?12 WHERE id = ?1",
                params![
                    data.id.to_string(),
                    data.debtor.id.to_string(),
                    data.paid_installments,
                    data.installments,
                    data.debtor.id.to_string(),
                    data.value,
                    data.title,
                    data.date_end.format("%d-%m-%Y").to_string(),
                    data.date_start.format("%d-%m-%Y").to_string(),
                    data.proof,
                    format!("{:?}", data.nature),
                    data.description
                ],
            )
            .map_err(|e| DatabaseError::ErrorOnEditData(e.to_string()))?;
        Ok(())
    }

    fn suspend(&self, data: Debt) -> Result<(), DatabaseError> {
        self.db
            .conn
            .execute(
                "DELETE FROM Counts WHERE id = ?1",
                params![data.id.to_string()],
            )
            .map_err(|e| DatabaseError::ErrorOnSuspendData(e.to_string()))?;
        Ok(())
    }

    fn get_data(&self, id: &str) -> Result<Debt, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT id, paid_installments, installments, value, title, date_out, date_in, proof, nature, description FROM Counts WHERE id = ?1")
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![id])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?
        {
            let id: String = row.get(0).unwrap();
            let date_out_str: String = row.get(5).unwrap();
            let date_in_str: String = row.get(6).unwrap();
            let nature_str: String = row.get(8).unwrap();

            Ok(Debt {
                id: id.parse().unwrap(),
                paid_installments: row.get(1).unwrap(),
                installments: row.get(2).unwrap(),
                value: row.get(3).unwrap(),
                title: row.get(4).unwrap(),
                date_end: NaiveDate::parse_from_str(&date_out_str, "%d-%m-%Y").unwrap(),
                date_start: NaiveDate::parse_from_str(&date_in_str, "%d-%m-%Y").unwrap(),
                proof: row.get(7).unwrap(),
                nature: match nature_str.as_str() {
                    "Health" => NatureDebt::Health,
                    "Home" => NatureDebt::Home,
                    "Transport" => NatureDebt::Transport,
                    "Food" => NatureDebt::Food,
                    "Investment" => NatureDebt::Investment,
                    "Incoming" => NatureDebt::Incoming,
                    _ => NatureDebt::Other("".to_string()),
                },
                description: row.get(9).unwrap(),
                debtor: People::default(),
                status: false,
            })
        } else {
            Err(DatabaseError::ErrorOnGetDataOfDatabase(
                "Debt not found".to_string(),
            ))
        }
    }

    fn get_all_from_user(&self, user_id: &str) -> Result<Vec<Debt>, DatabaseError> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT id, paid_installments, installments, value, title, date_out, date_in, proof, nature, description FROM Counts WHERE id_user = ?1")
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut rows = stmt
            .query(params![user_id])
            .map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))?;

        let mut debts = Vec::new();

        while let Some(row) = rows.next().map_err(|e| DatabaseError::ErrorOnGetDataOfDatabase(e.to_string()))? {
            let id: String = row.get(0).unwrap();
            let date_out_str: String = row.get(5).unwrap();
            let date_in_str: String = row.get(6).unwrap();
            let nature_str: String = row.get(8).unwrap();

            debts.push(Debt {
                id: id.parse().unwrap(),
                paid_installments: row.get(1).unwrap(),
                installments: row.get(2).unwrap(),
                value: row.get(3).unwrap(),
                title: row.get(4).unwrap(),
                date_end: NaiveDate::parse_from_str(&date_out_str, "%d-%m-%Y").unwrap(),
                date_start: NaiveDate::parse_from_str(&date_in_str, "%d-%m-%Y").unwrap(),
                proof: row.get(7).unwrap(),
                nature: match nature_str.as_str() {
                    "Health" => NatureDebt::Health,
                    "Home" => NatureDebt::Home,
                    "Transport" => NatureDebt::Transport,
                    "Food" => NatureDebt::Food,
                    "Investment" => NatureDebt::Investment,
                    "Incoming" => NatureDebt::Incoming,
                    _ => NatureDebt::Other("".to_string()),
                },
                description: row.get(9).unwrap(),
                debtor: People::default(),
                status: false,
            });
        }

        Ok(debts)
    }
}
