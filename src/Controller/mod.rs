pub mod config;
pub mod tests;

mod user_controller;
pub use user_controller::*;

mod data_controller;
pub use data_controller::*;

mod view_controller;
pub use view_controller::*;

mod file_controller;
pub use file_controller::*;

use crate::apis::send_email;
use crate::prelude::env::var;
use crate::prelude::log;
use crate::prelude::model::{errors::*, Count::Count, Database::*, List::ListCount, User::*};
use crate::prelude::segurance::*;
use crate::prelude::utils::{
    export::csv::export_csv, export::html::export_html, export::pdf::export_pdf,
    validate::alphabetic::is_alphabetic, validate::email_valid,
};
use crate::prelude::Instant;
use lazy_static::lazy_static;
use std::sync::Mutex;
