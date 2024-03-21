use crate::prelude::chrono::NaiveDate;

/// The `Info` struct represents information about a debtor.
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
/// let mut info = Info::new();
/// info.new_id();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
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

impl Info {
    /// Creates a new instance of the `Info` struct.
    ///
    /// The `id` field is initialized by concatenating the user's ID with a '0' and parsing it as an integer.
    /// The other fields are initialized with default values.
    ///
    /// # Returns
    ///
    /// An instance of the `Info` struct with initialized fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// let info = Info::new();
    /// ```
    pub fn new() -> Info {
        let today = chrono::Utc::now();

        Info {
            id: 0,
            debtor: String::new(),
            title: String::new(),
            description: String::new(),
            value: 0.0,
            date_in: today.date_naive(),
            date_out: today.date_naive(),
            paid_installments: 0,
            installments: 1,
            status: false,
            nature: String::from("Outros"),
        }
    }

    /// Generates a new ID for the debtor.
    ///
    /// The `id` field is incremented by 1.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut info = Info::new();
    /// info.new_id();
    /// ```
    pub fn new_id(&mut self) {
        self.id = self.id + 1;
    }
}
