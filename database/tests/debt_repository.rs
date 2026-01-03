use chrono::NaiveDate;
use esmeralda_database::{debt_repository::DebtRepository, rusqlite_impl::Db, Database};
use esmeralda_entities::{debt::{Debt, NatureDebt}, people::People};
use uuid::Uuid;

fn setup() -> (DebtRepository, Debt) {
    let db = Db::new().unwrap();
    let debt_repo = DebtRepository::new(db);
    let debt = Debt {
        id: Uuid::new_v4(),
        debtor: People {
            id: Uuid::new_v4(),
            name: "John".to_string(),
            surname: Some("Doe".to_string()),
            birthday: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            ..Default::default()
        },
        nature: NatureDebt::Food,
        date_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        date_end: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        installments: 12,
        paid_installments: 1,
        value: 1200.0,
        title: "Groceries".to_string(),
        description: "Monthly groceries".to_string(),
        status: true,
        ..Default::default()
    };
    (debt_repo, debt)
}

#[test]
fn test_insert_and_get_debt() {
    let (debt_repo, debt) = setup();

    debt_repo.insert(debt.clone()).unwrap();

    let fetched_debt = debt_repo.get_data(&debt.id.to_string()).unwrap();
    assert_eq!(debt, fetched_debt);
}

#[test]
fn test_edit_debt() {
    let (debt_repo, mut debt) = setup();

    debt_repo.insert(debt.clone()).unwrap();

    debt.title = "Supermarket".to_string();
    debt_repo.edit(debt.clone()).unwrap();

    let fetched_debt = debt_repo.get_data(&debt.id.to_string()).unwrap();
    assert_eq!(debt, fetched_debt);
}

#[test]
fn test_suspend_debt() {
    let (debt_repo, debt) = setup();

    debt_repo.insert(debt.clone()).unwrap();

    debt_repo.suspend(debt.clone()).unwrap();

    let result = debt_repo.get_data(&debt.id.to_string());
    assert!(result.is_err());
}
