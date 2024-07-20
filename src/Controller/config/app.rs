use crate::env::var;
use crate::gtk::prelude::*;
use crate::gtk::{Application, ApplicationWindow};

/// Configure the window of aplication and return him.
pub fn get_config(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_hexpand(true);
    window.set_title(Some("Esmeralda"));
    window.set_icon_name(Some(&format!("{}icon.ico", var("ICON_PATH").unwrap())));

    window
}
