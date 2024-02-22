pub use async_trait::async_trait;
pub use chrono;
pub use dioxus;
pub use dioxus::prelude::*;
pub use dioxus_desktop;
pub use dioxus_router;
pub use logger::log;
pub use thiserror::Error;
pub use tokio;

pub use std::env;
pub use std::error::Error;
pub use std::path::PathBuf;
pub use std::time::Instant;

/// APIs
#[path = "./APIs/mod.rs"]
pub mod router;

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
