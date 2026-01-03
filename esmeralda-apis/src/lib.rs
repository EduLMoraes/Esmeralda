use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Mailjet API error: {0}")]
    Mailjet(String),
    #[error("Yahoo Finance API error: {0}")]
    YahooFinance(String),
}

pub mod mailjet;
pub mod yahoo_finance;
