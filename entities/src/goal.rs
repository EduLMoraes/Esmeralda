use chrono::NaiveDate;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Goal {
    id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    pub start: NaiveDate,
    pub ui_desired_value: f64,
    pub ui_amount: f64,
    pub achivied: bool,
}

impl Goal {
    pub fn new(
        title: String,
        description: Option<String>,
        image: Option<String>,
        ui_desired_value: f64,
        ui_amount: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            image,
            start: chrono::Utc::now().date_naive(),
            ui_desired_value,
            ui_amount,
            achivied: false,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
