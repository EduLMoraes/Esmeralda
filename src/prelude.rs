/// Bibliotecas externas.
pub use dioxus;
pub use dioxus_desktop;

/// Módulos do projeto.
/// Controladores
#[path = "./controller/controller.rs"]
pub mod controller;

#[path = "./controller/errors.rs"]
pub mod errors;

#[path = "./controller/config.rs"]
pub mod config;


/// Telas de visualização
#[path = "./view/app.rs"]
pub mod app;