#[allow(unused_imports)]
use super::Count::Count;
#[allow(unused_imports)]
use super::NaiveDate;

// Creating a new Count with valid parameters should return a Count object with the correct values.
#[test]
fn test_new_count_with_valid_parameters() {
    let name = "John Doe";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert_eq!(count.debtor, String::from(name));
    assert_eq!(count.title, String::from(title));
    assert_eq!(count.description, String::from(desc));
    assert_eq!(count.value, value);
    assert_eq!(count.date_in, date_in);
    assert_eq!(count.paid_installments, 0);
    assert_eq!(count.installments, installments);
    assert!(!count.status);
    assert_eq!(count.nature, String::from(nature));
}

// Creating a new Count with an empty name field should return a Count object with an empty debtor field.
#[test]
fn test_new_count_with_empty_name_field() {
    let name = "";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert_eq!(count.debtor, "");
}

// Calling new_id() on a Count object should increment the id field by 1.
#[test]
fn test_new_id() {
    let mut count = Count::from(
        "John Doe",
        "Debt",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    let initial_id = count.id;
    count.new_id();
    assert_eq!(count.id, initial_id + 1);
}

// Calling pay_all() on a Count object with installments > 0 should set the paid_installments field to the same value as the installments field and set the status field to true.
#[test]
fn test_pay_all() {
    let mut count = Count::from(
        "John Doe",
        "Debt",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    count.pay_all();
    assert_eq!(count.paid_installments, count.installments);
    assert!(count.status);
}

// Calling pay() on a Count object with installments > 0 and paid_installments < installments should increment the paid_installments field by 1.
#[test]
fn test_pay() {
    let mut count = Count::from(
        "John Doe",
        "Debt",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    let initial_paid_installments = count.paid_installments;
    count.pay();
    assert_eq!(count.paid_installments, initial_paid_installments + 1);
}

// Calling pay() on a Count object with installments > 0 and paid_installments == installments - 1 should set the paid_installments field to the same value as the installments field and set the status field to true.
#[test]
fn test_pay_with_installments_minus_one() {
    let mut count = Count::from(
        "John Doe",
        "Debt",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    count.paid_installments = count.installments - 1;
    count.pay();
    assert_eq!(count.paid_installments, count.installments);
    assert!(count.status);
}

// Creating a new Count with a non-alphanumeric name field should return a Count object with an empty debtor field.
#[test]
fn test_new_count_with_non_alphanumeric_name() {
    let count = Count::from(
        "!@#$%",
        "Debt",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    assert_eq!(count.debtor, String::new());
}

// Creating a new Count with an empty title field should return a Count object with an empty title field.
#[test]
fn test_new_count_with_empty_title() {
    let count = Count::from(
        "John Doe",
        "",
        "Unpaid loan",
        1000.0,
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        12,
        "Loan",
    );
    assert_eq!(count.title, String::new());
}

// Creating a new Count with a value of 0.0 should return a Count object with a value field of 0.0.
#[test]
fn test_new_count_with_zero_value() {
    let name = "John Doe";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 0.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert_eq!(count.value, value);
}

// Calling pay() on a Count object with installments == 0 should not change the paid_installments or status fields.
#[test]
fn test_pay_with_zero_installments() {
    let name = "John Doe";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 0;
    let nature = "Loan";

    let mut count = Count::from(name, title, desc, value, date_in, installments, nature);
    count.pay();

    assert_eq!(count.paid_installments, 0);
    assert!(!count.status);
}

// Calling is_empty() on a Count object with an empty name field should return true.
#[test]
fn test_is_empty_with_empty_name() {
    let name = "";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert!(count.is_empty());
}

// Calling is_empty() on a Count object with a non-alphanumeric name field should return true.
#[test]
fn test_is_empty_with_non_alphanumeric_name() {
    let name = "John Doe!";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert!(count.is_empty());
}

// Calling is_empty() on a Count object with an empty title field should return true.
#[test]
fn test_is_empty_with_empty_title() {
    let name = "John Doe";
    let title = "";
    let desc = "Unpaid loan";
    let value = 1000.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert!(count.is_empty());
}

// Calling is_empty() on a Count object with a value of 0.0 should return true.
#[test]
fn test_is_empty_with_zero_value() {
    let name = "John Doe";
    let title = "Debt";
    let desc = "Unpaid loan";
    let value = 0.0;
    let date_in = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let installments = 12;
    let nature = "Loan";

    let count = Count::from(name, title, desc, value, date_in, installments, nature);

    assert!(count.is_empty());
}

// Calling to_string() on a Count object should return a string representation of the object in the format "id, debtor, title, description, value, date_in, date_out, paid_installments, installments, nature, status".
#[test]
fn test_to_string() {
    let count = Count {
        id: 0,
        debtor: String::from("John"),
        title: String::from("Loan"),
        description: String::from("Loan for car"),
        value: 10000.0,
        date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        paid_installments: 0,
        installments: 12,
        status: false,
        nature: String::from("Loan"),
    };

    let expected =
        "0, John, Loan, Loan for car, 10000.00, 2022-01-01, 2023-01-01, 0, 12, Loan, false";
    let result = count.to_string();

    assert_eq!(result, expected);
}
