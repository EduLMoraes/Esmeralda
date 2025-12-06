use super::ApiError;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;

use async_trait::async_trait;

// ... (rest of the imports)

#[async_trait]
pub trait MailjetApi {
    async fn send_email(
        &self,
        sender: &str,
        recipient: &str,
        title: &str,
        text: String,
        subject: String,
    ) -> Result<(), ApiError>;
}

pub struct MailjetApiImpl;

#[async_trait]
impl MailjetApi for MailjetApiImpl {
    async fn send_email(
        &self,
        sender: &str,
        recipient: &str,
        title: &str,
        text: String,
        subject: String,
    ) -> Result<(), ApiError> {
        // FIXME: Using environment variables for keys is insecure.
        let client = Client::new(
            SendAPIVersion::V3,
            &env::var("KEYMAILAPI").map_err(|e| ApiError::Mailjet(e.to_string()))?,
            &env::var("KEYPRIVATE").map_err(|e| ApiError::Mailjet(e.to_string()))?,
        );

        let mut message = Message::new(sender, title, Some(subject), Some(text));
        message.push_recipient(Recipient::new(recipient));

        client
            .send(message)
            .await
            .map(|_| ())
.map_err(|e| ApiError::Mailjet(format!("{:?}", e)))
    }
}
