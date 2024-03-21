use crate::env::var;
use crate::gtk::prelude::*;
use crate::gtk::{Application, ApplicationWindow};

pub fn get_config(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);

    window.set_hexpand(false);
    // window.set_fullscreened(true);
    window.set_default_width(1280);
    window.set_default_height(720);
    window.set_resizable(false);

    window.set_title(Some("Esmeralda"));
    window.set_icon_name(Some(&format!("{}icon.ico", var("ICON_PATH").unwrap())));

    window
}
