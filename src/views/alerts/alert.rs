use super::*;

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
                .modal(true)
                .focus_on_click(false)
                .overflow(gtk::Overflow::Visible)
                .focus_visible(false)
                .focusable(false)
                .mnemonics_visible(true)
                .sensitive(true)
                .use_markup(true)
                .hide_on_close(true)
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
