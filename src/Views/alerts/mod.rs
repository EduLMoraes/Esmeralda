use super::*;
use crate::chrono::Datelike;
#[allow(deprecated)]
use crate::gtk::{Adjustment, Calendar, CheckButton, DropDown, MessageDialog, SpinButton};
use crate::model::Count::Count;

mod edit;
pub use edit::edit_count;
static mut HAS_MESSAGE_DIALOG: bool = false;

#[allow(deprecated)]
pub fn alert(message: &str, title: &str) {
    unsafe {
        if !HAS_MESSAGE_DIALOG {
            let alert_dialog = MessageDialog::builder()
                .message_type(gtk::MessageType::Warning)
                .text(message)
                .title(title)
                .width_request(350)
                .height_request(70)
                .buttons(gtk::ButtonsType::Ok)
                .build();

            alert_dialog.connect_response(move |alert, _| {
                alert.destroy();
                HAS_MESSAGE_DIALOG = false;
            });

            HAS_MESSAGE_DIALOG = true;

            alert_dialog.show();
        }
    }
}

#[allow(deprecated)]
pub fn confirm(message: &str, title: &str) -> Option<MessageDialog> {
    unsafe {
        if !HAS_MESSAGE_DIALOG {
            let dialog = MessageDialog::builder()
                .buttons(gtk::ButtonsType::YesNo)
                .message_type(gtk::MessageType::Warning)
                .text(message)
                .title(title)
                .build();

            dialog.connect_destroy(|_| {
                HAS_MESSAGE_DIALOG = false;
            });
            HAS_MESSAGE_DIALOG = true;

            Some(dialog)
        } else {
            None
        }
    }
}
