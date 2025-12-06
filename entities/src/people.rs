use chrono::NaiveDate;
use crate::Data;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct People {
    pub id: Uuid,
    pub rg: Option<String>,
    pub cpf: Option<String>,
    pub name: String,
    pub surname: Option<String>,
    pub wage: Option<f64>,
    pub cell_phone: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub birthday: NaiveDate,
    pub voter_registration: Option<String>,
    pub address: Option<String>,
    pub provider: Option<String>,
}

impl Data for People {
    const TITLE: &'static str = "people";
}

