use super::ApiError;

pub trait MailjetApi {
    fn send_email(
        &self,
        sender: &str,
        recipient: &str,
        title: &str,
        text: String,
        subject: String,
    ) -> Result<(), ApiError>;
}

pub struct MailjetApiImpl;

impl MailjetApi for MailjetApiImpl {
    fn send_email(
        &self,
        _sender: &str,
        _recipient: &str,
        _title: &str,
        _text: String,
        _subject: String,
    ) -> Result<(), ApiError> {
        unimplemented!()
    }
}
