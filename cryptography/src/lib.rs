#[derive(thiserror::Error, Debug)]
pub enum CryptographerError {
    #[error("Error to cryptographer information: {0}")]
    ErrorToProtectMessage(String),

    #[error("Error to descroptography information: {0}")]
    ErrorToDescryptograpyMessage(String),
}

pub trait Cryptographer {
    fn protect(&self, message: &str) -> Result<String, CryptographerError>;
    fn descryptography(&self, message: &str) -> Result<String, CryptographerError>;
}
