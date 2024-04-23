use super::*;
use crate::gtk::{AlertDialog, MessageDialog, ResponseType, Window};

pub fn alert(message: &str, title: &str) {
    let window_alert = Window::new();
    window_alert.set_title(Some(title));
    window_alert.set_default_size(350, 70);

    let alert_dialog = AlertDialog::builder().message(message).build();

    alert_dialog.show(Some(&window_alert));
}

#[allow(unused)]
pub fn confirm(message: &str, title: &str) -> MessageDialog {
    let confirm = MessageDialog::builder()
        .buttons(gtk::ButtonsType::YesNo)
        .message_type(gtk::MessageType::Warning)
        .text(message)
        .title(title)
        .build();

    confirm
}
