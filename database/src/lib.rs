use esmeralda_entities::Data;

pub mod debt_repository;
pub mod investment_repository;
pub mod people_repository;
pub mod rusqlite_impl;
pub mod user_repository;

pub use debt_repository::DebtRepository;
pub use investment_repository::InvestmentRepository;
pub use people_repository::PeopleRepository;
pub use rusqlite_impl::Db;
pub use user_repository::UserRepository;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Error on insert data on database: {0}")]
    ErrorOnInsertData(String),

    #[error("Error on edit data on database: {0}")]
    ErrorOnEditData(String),

    #[error("Error on suspend data on database: {0}")]
    ErrorOnSuspendData(String),

    #[error("Error on get data of database: {0}")]
    ErrorOnGetDataOfDatabase(String),
}

pub trait Database<D: Data + serde::Serialize + serde::de::DeserializeOwned + Sized> {
    fn insert(&self, data: D) -> Result<(), DatabaseError>;
    fn edit(&self, data: D) -> Result<(), DatabaseError>;
    fn suspend(&self, data: D) -> Result<(), DatabaseError>;
    fn get_data(&self, id: &str) -> Result<D, DatabaseError>;
    fn get_by_email(&self, _email: &str) -> Result<D, DatabaseError> {
        Err(DatabaseError::ErrorOnGetDataOfDatabase(
            "Method not implemented".to_string(),
        ))
    }
    fn get_all_from_user(&self, _user_id: &str) -> Result<Vec<D>, DatabaseError> {
        Err(DatabaseError::ErrorOnGetDataOfDatabase(
            "Method not implemented".to_string(),
        ))
    }
}
