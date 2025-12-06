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

pub trait Data {
    const TITLE: String;

    fn get_title(&self) -> String {
        Self::TITLE
    }
}

pub trait Database<'a, D: Data + serde::Serialize + serde::Deserializer<'a> + Sized> {
    fn insert(&self, data: D) -> Result<(), DatabaseError>;
    fn edit(&self, data: D) -> Result<(), DatabaseError>;
    fn suspend(&self, data: D) -> Result<(), DatabaseError>;
    fn get_data(&self, id: &'a str) -> Result<D, DatabaseError>;
}
