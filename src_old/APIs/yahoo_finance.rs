use yahoo::time::OffsetDateTime;
use yahoo::YahooError;
use yahoo::{Dividend, Quote};
use yahoo_finance_api as yahoo;

pub fn get_quote(quote: &str) -> Result<(Quote, Option<Dividend>), YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let now = OffsetDateTime::now_utc();
    let last_year = now.replace_year(now.year() - 1).unwrap_or(now);
    let response = provider.get_quote_history_interval(quote, last_year, now, "1mo")?;
    if !response.dividends()?.is_empty(){
        Ok((
            response.last_quote().unwrap(),
            Some(response.dividends().unwrap()[0].clone()),
        ))
    } else {
        Ok((response.last_quote().unwrap(), None))
    }
}
