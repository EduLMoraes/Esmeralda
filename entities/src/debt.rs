use crate::people::People;
use chrono::NaiveDate;
use crate::Data;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Debt {
    pub id: Uuid,
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
    pub value: f64,
    pub proof: Option<String>,
    pub title: String,
    pub description: String,
    pub status: bool,
}

impl Data for Debt {
    const TITLE: &'static str = "debts";
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum NatureDebt {
    Health,
    Home,
    Transport,
    Food,
    Investment,
    Incoming,
    Other(String),
}

impl Default for NatureDebt {
    fn default() -> Self {
        NatureDebt::Other("".to_string())
    }
}

impl Default for Debt {
    fn default() -> Self {
        Debt {
            id: Uuid::new_v4(),
            debtor: People::default(),
            nature: NatureDebt::default(),
            date_start: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_end: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            installments: 0,
            paid_installments: 0,
            value: 0.0,
            proof: None,
            title: "".to_string(),
            description: "".to_string(),
            status: false,
        }
    }
}
