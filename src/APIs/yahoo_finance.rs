use yahoo::YahooError;
use yahoo::{Dividend, Quote};
use yahoo_finance_api as yahoo;

pub fn get_quote(quote: &str) -> Result<(Quote, Option<Dividend>), YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes(quote, "3mo")?;
    if response.dividends()?.len() > 0 {
        Ok((
            response.last_quote().unwrap(),
            Some(response.dividends().unwrap()[0].clone()),
        ))
    } else {
        Ok((response.last_quote().unwrap(), None))
    }
}
