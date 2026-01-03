use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serializer};

pub trait Data {
    const TITLE: &'static str;

    fn get_title(&self) -> &'static str {
        Self::TITLE
    }
}

pub mod debt;
pub mod goal;
pub mod investment;
pub mod people;
pub mod user;

const FORMAT: &str = "%d-%m-%Y";

pub fn serialize_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format(FORMAT).to_string())
}

pub fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(de::Error::custom)
}
