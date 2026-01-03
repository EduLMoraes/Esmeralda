use chrono::NaiveDate;
use esmeralda_database::{investment_repository::InvestmentRepository, rusqlite_impl::Db, Database};
use esmeralda_entities::{
    investment::{Investment, InvestmentType, StockExchangeShares},
    people::People,
};
use uuid::Uuid;

fn setup() -> (InvestmentRepository, Investment) {
    let db = Db::new().unwrap();
    let investment_repo = InvestmentRepository::new(db);
    let investment = Investment {
        id: Uuid::new_v4(),
        owner: People {
            id: Uuid::new_v4(),
            name: "John".to_string(),
            surname: Some("Doe".to_string()),
            birthday: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            ..Default::default()
        },
        investment_type: InvestmentType::Stock(StockExchangeShares {
            id: Uuid::new_v4(),
            value: 100.0,
        }),
        value: 1000.0,
        redemption_value: 1100.0,
        date_apply: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        symbol: "APPL".to_string(),
        title: "Apple Inc.".to_string(),
        n_quotas: 10,
        cnpj: "00.000.000/0000-00".to_string(),
    };
    (investment_repo, investment)
}

#[test]
fn test_insert_and_get_investment() {
    let (investment_repo, investment) = setup();

    investment_repo.insert(investment.clone()).unwrap();

    let fetched_investment = investment_repo
        .get_data(&investment.id.to_string())
        .unwrap();
    assert_eq!(investment, fetched_investment);
}

#[test]
fn test_edit_investment() {
    let (investment_repo, mut investment) = setup();

    investment_repo.insert(investment.clone()).unwrap();

    investment.title = "Apple".to_string();
    investment_repo.edit(investment.clone()).unwrap();

    let fetched_investment = investment_repo
        .get_data(&investment.id.to_string())
        .unwrap();
    assert_eq!(investment, fetched_investment);
}

#[test]
fn test_suspend_investment() {
    let (investment_repo, investment) = setup();

    investment_repo.insert(investment.clone()).unwrap();

    investment_repo.suspend(investment.clone()).unwrap();

    let result = investment_repo.get_data(&investment.id.to_string());
    assert!(result.is_err());
}
