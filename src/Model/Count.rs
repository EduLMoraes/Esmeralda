use chrono::Datelike;

use crate::prelude::chrono::NaiveDate;

/// The `Count` struct represents information about a debtor.
///
/// # Fields
///
/// - `id`: An integer representing the ID of the debtor.
/// - `debtor`: A string representing the name of the debtor.
/// - `title`: A string representing the title of the debtor.
/// - `description`: A string representing the description of the debtor.
/// - `value`: A float representing the value of the debtor.
/// - `date_in`: A `NaiveDate` representing the date the debtor was created.
/// - `date_out`: A `NaiveDate` representing the date the debtor is due.
/// - `paid_installments`: An unsigned integer representing the number of paid installments.
/// - `installments`: An unsigned integer representing the total number of installments.
/// - `status`: A boolean representing the status of the debtor.
///
/// # Example Usage
///
/// ```rust
/// let mut count = Count::new();
/// count.new_id();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Count {
    pub id: i32,
    pub debtor: String,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub date_in: NaiveDate,
    pub date_out: NaiveDate,
    pub paid_installments: u32,
    pub installments: u32,
    pub status: bool,
    pub nature: String,
}

impl Count {
    pub fn from(
        name: &str,
        title: &str,
        desc: &str,
        value: f32,
        date_in: NaiveDate,
        installments: u32,
        nature: &str,
    ) -> Count {
        Count {
            id: 0,
            debtor: String::from(name),
            title: String::from(title),
            description: String::from(desc),
            value: value,
            date_in: date_in,
            date_out: {
                let mut tmp_month = date_in.month() + installments;
                let mut tmp_year = date_in.year();
                let mut tmp_installments = tmp_month;

                while tmp_month > 12 {
                    tmp_year += 1;
                    tmp_installments -= 12;
                    tmp_month = tmp_installments;
                }

                NaiveDate::from_ymd_opt(tmp_year, tmp_month, date_in.day() as u32).unwrap()
            },
            paid_installments: 0,
            installments: installments,
            status: false,
            nature: String::from(nature),
        }
    }

    /// Generates a new ID for the debtor.
    ///
    /// The `id` field is incremented by 1.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut count = Count::new();
    /// count.new_id();
    /// ```
    pub fn new_id(&mut self) {
        self.id = self.id + 1;
    }

    pub fn pay_all(&mut self) {
        self.paid_installments = self.installments;
        self.status = true;
    }

    pub fn pay(&mut self) {
        if self.paid_installments + 1 == self.installments {
            self.pay_all();
        } else if self.paid_installments < self.installments {
            self.paid_installments += 1;
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {:.2}, {}, {}, {}, {}, {}, {}",
            self.id,
            self.debtor,
            self.title,
            self.description,
            self.value,
            self.date_in,
            self.date_out,
            self.paid_installments,
            self.installments,
            self.nature,
            self.status
        )
    }

    pub fn is_empty(&self) -> bool {
        self.debtor.is_empty() || self.title.is_empty()
    }
}
