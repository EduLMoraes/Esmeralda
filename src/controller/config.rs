use crate::dioxus_desktop::{Config, WindowBuilder};

pub fn config() -> Config{
    Config::default().with_window(WindowBuilder::new().with_resizable(true).with_inner_size(
        dioxus_desktop::wry::application::dpi::LogicalSize::new(400.0, 300.0),
    ))
}