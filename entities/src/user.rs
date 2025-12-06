use chrono::NaiveDate;
use crate::Data;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub last_login: NaiveDate,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    pub username: String,
}

impl Data for User {
    const TITLE: &'static str = "users";
}
