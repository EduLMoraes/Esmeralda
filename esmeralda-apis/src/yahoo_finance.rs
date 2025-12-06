use super::ApiError;
use yahoo_finance_api as yahoo;
use time::OffsetDateTime;
use yahoo::YahooConnector;
use async_trait::async_trait;

#[async_trait]
pub trait YahooFinanceApi {
    async fn get_quote(&self, quote: &str) -> Result<(yahoo::Quote, Option<yahoo::Dividend>), ApiError>;
}

pub struct YahooFinanceApiImpl {
    provider: YahooConnector,
}

impl YahooFinanceApiImpl {
    pub fn new() -> Self {
        let provider = YahooConnector::new();
        Self { provider }
    }
}

#[async_trait]
impl YahooFinanceApi for YahooFinanceApiImpl {
    async fn get_quote(&self, quote: &str) -> Result<(yahoo::Quote, Option<yahoo::Dividend>), ApiError> {
        let now = OffsetDateTime::now_utc();
        let last_year = now.replace_year(now.year() - 1).unwrap_or(now);
        let response = self.provider.get_quote_history_interval(quote, last_year, now, "1mo").await?;

        let last_quote = response.last_quote()?;

        if let Ok(dividends) = response.dividends() {
            if !dividends.is_empty() {
                return Ok((last_quote, Some(dividends[0].clone())));
            }
        }
        
        Ok((last_quote, None))
    }
}

impl From<yahoo::YahooError> for ApiError {
    fn from(error: yahoo::YahooError) -> Self {
        ApiError::YahooFinance(error.to_string())
    }
}
