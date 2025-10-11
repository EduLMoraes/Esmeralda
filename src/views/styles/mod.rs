use dotenvy::var;
use gtk::{gdk::Display, CssProvider};

pub fn load_style() {
    let provider = CssProvider::new();
    provider.load_from_path(var("CSS_PATH").unwrap());

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}
