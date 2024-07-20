pub use ::regex;
pub use chrono;
pub use gtk;
pub use gtk::prelude::*;
pub use gtk::{gdk::Display, Application};
pub use logger::log;
pub use std;
pub use thiserror::Error;
pub use tokio;

pub use std::env;
pub use std::error::Error;
pub use std::time::Instant;

/// APIs
#[path = "./APIs/mod.rs"]
pub mod apis;

/// Controllers
#[path = "./Controller/mod.rs"]
pub mod control;

/// Models
#[path = "Model/mod.rs"]
pub mod model;

/// Segurance
#[path = "Segurance/mod.rs"]
pub mod segurance;

/// Utilities
#[path = "Utils/mod.rs"]
pub mod utils;

#[path = "Views/mod.rs"]
pub mod views;

#[path = "Views/styles/sty.rs"]
pub mod sty;
