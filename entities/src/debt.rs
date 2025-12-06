use crate::people::People;
use chrono::NaiveDate;

#[derive(thiserror::Error, Debug)]
pub enum DebtsControlError {
    #[error("Error on pay all")]
    ErrorPayAll,
}

pub trait DebtsControl {
    fn pay_all(&self) -> Result<(), DebtsControlError>;
    fn pay_one(&self) -> Result<(), DebtsControlError>;
    fn list_all(&self) -> Result<Vec<Debt>, DebtsControlError>;
    fn get_debt(&self, id: String) -> Result<Debt, DebtsControlError>;
    fn edit_one(&self, id: String) -> Result<(), DebtsControlError>;
    fn search(&self, key_word: String) -> Result<Vec<Debt>, DebtsControlError>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Debt {
    id: String,
    pub debtor: People,
    pub nature: NatureDebt,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub date_start: NaiveDate,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub date_end: NaiveDate,
    pub installments: u16,
    pub paid_installments: u16,
    pub ui_value: f64,
    pub proof: Option<String>,
    pub title: String,
    pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum NatureDebt {
    Health,
    Home,
    Transport,
    Food,
    Investment,
    Incoming,
    Other(String),
}
