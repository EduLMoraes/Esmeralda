use crate::prelude::model::list::InterfaceInfo;
use crate::prelude::dioxus::prelude::*;
use crate::prelude::chrono::{NaiveDate, Months};
use crate::prelude::PathBuf;
use crate::prelude::log;

use crate::prelude::control;

use crate::prelude::model::{
    Info::Info,
    Message::Message
};

pub mod div_active;
pub mod div_add;
pub mod div_edit;
pub mod div_export;
pub mod div_options;
pub mod div_paid;

/// Represents a struct called `Columns` with several boolean fields.
///
/// # Fields
/// - `name`: Represents the presence of the name column.
/// - `title`: Represents the presence of the title column.
/// - `description`: Represents the presence of the description column.
/// - `date_in`: Represents the presence of the date_in column.
/// - `date_out`: Represents the presence of the date_out column.
/// - `paid_installments`: Represents the presence of the paid_installments column.
/// - `installments`: Represents the presence of the installments column.
/// - `value`: Represents the presence of the value column.
/// - `status`: Represents the presence of the status column.
#[derive(Clone, Debug)]
pub struct Columns {
    pub name: bool,
    pub title: bool,
    pub description: bool,
    pub date_in: bool,
    pub date_out: bool,
    pub installments: bool,
    pub value: bool,
    pub status: bool,
    pub nature: bool,
}

impl Columns {
    /// Creates a new instance of the `Columns` struct with default field values.
    ///
    /// # Example
    ///
    /// ```
    /// let columns = Columns::new();
    /// ```
    ///
    /// # Returns
    /// A new instance of the `Columns` struct with default field values.
    pub fn new() -> Columns {
        Columns {
            name: true,
            title: true,
            description: false,
            date_in: false,
            date_out: true,
            installments: false,
            value: true,
            status: true,
            nature: false,
        }
    }
}

/// Represents the contabilized status of something.
#[derive(PartialEq)]
pub enum Contabilized {
    Yes = 1,
    No = 0,
}
