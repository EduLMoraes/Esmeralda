use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};

pub async fn send_email(
    sender: &str,
    recipient: &str,
    title: &str,
    text: String,
    subject: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(
        SendAPIVersion::V3,
        "b96e5a5ba71810ca3dc52a4e9db40547",
        "df70dc61c3bc7e66185604d09307e1ef",
    );

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(sender, title, Some(subject), Some(text));

    message.push_recipient(Recipient::new(recipient));
    let _ = client.send(message).await;
    Ok(())
}
