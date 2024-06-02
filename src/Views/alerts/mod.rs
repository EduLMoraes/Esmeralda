use super::*;
use crate::chrono::Datelike;
#[allow(deprecated)]
use crate::gtk::{
    Adjustment, AlertDialog, Calendar, CheckButton, DropDown, MessageDialog, SpinButton, Window,
};
use crate::model::Count::Count;

mod edit;
pub use edit::edit_count;

pub fn alert(message: &str, title: &str) {
    let window_alert = Window::new();
    window_alert.set_title(Some(title));
    window_alert.set_default_size(350, 70);

    let alert_dialog = AlertDialog::builder().message(message).build();

    alert_dialog.show(Some(&window_alert));
}

#[allow(deprecated)]
pub fn confirm(message: &str, title: &str) -> MessageDialog {
    let confirm = MessageDialog::builder()
        .buttons(gtk::ButtonsType::YesNo)
        .message_type(gtk::MessageType::Warning)
        .text(message)
        .title(title)
        .build();

    confirm
}
