pub use chrono;
pub use dioxus;
pub use dioxus::prelude::*;
pub use dioxus_desktop;
pub use dioxus_router;
pub use thiserror::Error;
pub use tokio;

pub use std::env::var;
pub use std::error::Error;
pub use std::time::Instant;

#[path = "./controller/control.rs"]
pub mod control;

#[path = "./controller/router.rs"]
pub mod router;

#[path = "./controller/utils/cryptography.rs"]
pub mod cryptography;

#[path = "./controller/utils/email_valid.rs"]
pub mod email_valid;

#[path = "./controller/utils/compare_dates.rs"]
pub mod compare_dates;

#[path = "./controller/utils/export.rs"]
pub mod export;

#[path = "./controller/utils/move_pages.rs"]
pub mod move_pages;

#[path = "./controller/utils/alphabetic.rs"]
pub mod alphabetic;

#[path = "./controller/config/to_db.rs"]
pub mod to_db;

#[path = "./controller/config/to_app.rs"]
pub mod to_app;

#[path = "./controller/database/db.rs"]
pub mod db;

#[path = "./model/app.rs"]
pub mod app;

#[path = "./model/errors.rs"]
pub mod errors;

#[path = "./model/structs_db.rs"]
pub mod structs_db;

#[path = "./model/structs.rs"]
pub mod structs;
