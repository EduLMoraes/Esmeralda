use crate::prelude::dioxus_desktop::{Config, WindowBuilder};
use std::path::PathBuf;

/// Returns a `Config` object with specified properties set.
///
/// # Example
///
/// ```
/// let config = get_config();
/// ```
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// A `Config` object with the specified properties set.
#[allow(dead_code)]
pub fn get_config() -> Config {
    let window = WindowBuilder::new();
    let window = window.with_title("Esmeralda");
    let window = window.with_transparent(false);
    let window = window.with_maximized(true);

    Config::new()
        .with_window(window)
        .with_disable_context_menu(false)
        .with_resource_directory(PathBuf::from("./src/"))
}

