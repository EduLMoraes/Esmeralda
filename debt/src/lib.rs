use esmeralda_entities::debt::Debt;

pub mod debt_service;
pub use debt_service::DebtServiceImpl;

#[derive(thiserror::Error, Debug)]
pub enum DebtError {
    #[error("Error on pay of Debt")]
    ErrorToPay,
}

pub trait DebtService {
    fn get_all(&self, user_id: &str) -> Result<Vec<Debt>, DebtError>;
    fn insert(&self, debt: Debt) -> Result<(), DebtError>;
    fn pay_installment(&self, debt: &mut Debt) -> Result<(), DebtError>;
    fn pay_all(&self, debt: &mut Debt) -> Result<(), DebtError>;
    fn calculate_end_date(&self, debt: &Debt) -> chrono::NaiveDate;
}