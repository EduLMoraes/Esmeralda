use super::*;
use crate::gtk::{AlertDialog, Window};

pub fn alert(message: &str, title: &str) {
    let window_alert = Window::new();
    window_alert.set_title(Some(title));
    window_alert.set_default_size(350, 70);

    let alert_dialog = AlertDialog::builder().message(message).build();

    alert_dialog.show(Some(&window_alert));
}
