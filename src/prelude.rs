/// Lib extern.
pub use dioxus;
pub use dioxus_desktop;
pub use dioxus_router;
pub use dioxus::prelude::*;
pub use chrono;
pub use thiserror::Error;
pub use tokio;

/// Importações do rust;
pub use std::env::var;
pub use std::error::Error;

/// <h1>Modules of project.</h1>

/// <h2>Controller</h2>
#[path = "./controller/control.rs"]
pub mod control;

/// <h3>Router</h3>
#[path = "./controller/router.rs"]
pub mod router;

/// <h3>Utils</h3>
#[path = "./controller/utils/cryptography.rs"]
pub mod cryptography;

#[path = "./controller/utils/email_valid.rs"]
pub mod email_valid;

#[path = "./controller/utils/export.rs"]
pub mod export;

#[path = "./controller/utils/move_pages.rs"]
pub mod move_pages;

#[path = "./controller/utils/alphabetic.rs"]
pub mod alphabetic;

/// <h3>Configurations</h3>
#[path = "./controller/config/to_db.rs"]
pub mod to_db;

#[path = "./controller/config/to_app.rs"]
pub mod to_app;

/// <h3>Database</h3>
#[path = "./controller/database/db.rs"]
pub mod db;

/// <h2>Model</h2>
#[path = "./model/app.rs"]
pub mod app;

#[path = "./model/errors.rs"]
pub mod errors;

#[path = "./model/structs_db.rs"]
pub mod structs_db;

#[path = "./model/structs.rs"]
pub mod structs;
