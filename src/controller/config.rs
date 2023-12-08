use std::path::PathBuf;
use crate::dioxus_desktop::{Config, WindowBuilder};

pub fn config() -> Config{
    let window = WindowBuilder::new();
    let window = window.with_title("Esmeralda");

    Config::new()
        .with_window(window)
        .with_disable_context_menu(true)
        .with_resource_directory(PathBuf::from("./src/"))
}