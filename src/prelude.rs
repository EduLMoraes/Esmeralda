/// Bibliotecas externas.
pub use dioxus;
pub use dioxus_desktop;
pub use dioxus_router;
pub use chrono;
pub use thiserror::Error;
pub use tokio;

/// Importações do rust;
pub use std::env::var;
pub use std::error::Error;

/// Módulos do projeto.
/// Controladores
#[path = "./controller/controller.rs"]
pub mod controller;

#[path = "./controller/export.rs"]
pub mod export;

#[path = "./controller/crypt.rs"]
pub mod crypt;

#[path = "./controller/config.rs"]
pub mod config;

#[path = "./controller/db.rs"]
pub mod db;

/// Modelos para visualização
#[path = "./model/errors.rs"]
pub mod errors;

#[path = "./model/structs.rs"]
pub mod structs;

/// Telas de visualização
#[path = "./view/app.rs"]
pub mod app;
