use crate::dioxus_desktop::{Config, WindowBuilder};
use std::path::PathBuf;

pub fn config() -> Config {
    let window = WindowBuilder::new();
    let window = window.with_title("Esmeralda");
    let window = window.with_transparent(false);

    Config::new()
        .with_window(window)
        .with_disable_context_menu(false)
        .with_resource_directory(PathBuf::from("./src/"))
}
