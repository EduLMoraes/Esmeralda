use esmeralda_entities::Data;

pub mod rusqlite_impl;
pub mod user_repository;
pub mod people_repository;
pub mod debt_repository;
pub mod investment_repository;

pub use rusqlite_impl::Db;
pub use user_repository::UserRepository;
pub use people_repository::PeopleRepository;
pub use debt_repository::DebtRepository;
pub use investment_repository::InvestmentRepository;

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

pub trait Database<'a, D: Data + serde::Serialize + serde::de::DeserializeOwned + Sized> {
    fn insert(&self, data: D) -> Result<(), DatabaseError>;
    fn edit(&self, data: D) -> Result<(), DatabaseError>;
    fn suspend(&self, data: D) -> Result<(), DatabaseError>;
    fn get_data(&self, id: &'a str) -> Result<D, DatabaseError>;
    fn get_by_email(&self, email: &'a str) -> Result<D, DatabaseError> {
        Err(DatabaseError::ErrorOnGetDataOfDatabase("Method not implemented".to_string()))
    }
}
