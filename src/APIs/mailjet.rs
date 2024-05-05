use crate::env::var;
use crate::model::errors::{ApiError, ErrorLog};
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

// A API não está funcionando corretamente!
#[allow(deprecated, unused)]
pub async fn send_email(
    sender: &str,
    recipient: &str,
    title: &str,
    text: String,
    subject: String,
) -> Result<(), ApiError> {
    let client = Client::new(
        SendAPIVersion::V3,
        &var("KEYMAILAPI").unwrap(),
        &var("KEYPRIVATE").unwrap(),
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(sender, title, Some(subject), Some(text));

    message.push_recipient(Recipient::new(recipient));
    let response = client.send(message).await;

    match response {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError::ErrorMailjet(ErrorLog {
            title: "Error to send email",
            code: 500,
            file: "mailjet.rs",
        })),
    }
}
