use super::People::People;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct InvestmentsFiisStockExchangeShares {
    fii: InvestmentFii,
    id_ses: StockExchangeShares,
    investment: Investment,
    n_quotas: u32,
    simbol: String,
    title: String,
}

#[derive(Clone, Debug)]
enum TypeInvestment {
    Fii(InvestmentFii),
}

#[derive(Clone, Debug)]
pub struct Investment {
    id: u32,
    owner: People,
    type_inv: TypeInvestment,
    value_aply: f32,
    redeption_value: f32,
    date_apply: NaiveDate,
    investiment: String,
    cnpj: String,
}

#[derive(Clone, Debug)]
pub struct StockExchangeShares {
    id: u32,
    value: f32,
}

#[derive(Clone, Debug)]
pub struct InvestmentFii {
    id: u32,
    last_yields: Vec<LastYields>,
    dates_yields: Vec<DatesYields>,
    value: f32,
    dividend_yield: f32,
}

#[derive(Clone, Debug)]
pub struct LastYields {
    id: u32,
    value: f32,
}

#[derive(Clone, Debug)]
pub struct DatesYields {
    id: u32,
    date: NaiveDate,
}
