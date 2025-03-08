use super::*;

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

            dialog.connect_response(|alert, _| {
                alert.destroy();
                HAS_MESSAGE_DIALOG = false;
            });

            HAS_MESSAGE_DIALOG = true;

            Some(dialog)
        } else {
            tracing::error!("Failed to create confirm alert");
            None
        }
    }
}
