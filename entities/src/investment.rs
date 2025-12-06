use crate::people::People;
use chrono::NaiveDate;
use crate::Data;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Investment {
    pub id: Uuid,
    pub owner: People,
    pub investment_type: InvestmentType,
    pub value: f64,
    pub redemption_value: f64,
    pub date_apply: NaiveDate,
    pub symbol: String,
    pub title: String,
    pub n_quotas: u32,
    pub cnpj: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum InvestmentType {
    Fii(InvestmentFii),
    Stock(StockExchangeShares),
}

impl Default for InvestmentType {
    fn default() -> Self {
        InvestmentType::Stock(StockExchangeShares::default())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct StockExchangeShares {
    pub id: Uuid,
    pub value: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct InvestmentFii {
    pub id: Uuid,
    pub last_yields: Vec<LastYields>,
    pub dates_yields: Vec<DatesYields>,
    pub value: f64,
    pub dividend_yield: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct LastYields {
    pub id: Uuid,
    pub value: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct DatesYields {
    pub id: Uuid,
    pub date: NaiveDate,
}

impl Data for Investment {
    const TITLE: &'static str = "investments";
}
