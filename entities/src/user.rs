use chrono::NaiveDate;

use crate::people::People;

#[derive(thiserror::Error, Debug)]
pub enum UserControlError {
    #[error("Failed to transform into a people")]
    ErrorToTransformInPeople,
}

pub trait UserControl {
    fn new(email: String, password: String, username: String) -> Self;
    fn get_id(&self) -> u64;
    fn get_people(&self) -> Result<People, UserControlError>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct User {
    id: String,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub last_login: NaiveDate,
    pub email: String,
    password: String,
    username: String,
    people: Vec<People>,
}
