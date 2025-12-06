#[derive(thiserror::Error, Debug)]
pub enum DebtError {
    #[error("Error on pay of Debt")]
    ErrorToPay,

    #[error("Error on edit Debt")]
    ErrorToEdit,
}

pub trait Debt {
    fn pay(&mut self) -> Result<(), DebtError>;
    fn edit(&mut self) -> Result<(), DebtError>;
}
