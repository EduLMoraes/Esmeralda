#[allow(unused_imports)]
pub use super::*;

// Returns true if all fields in Count struct are non-empty and valid.
// #[tokio::test]
async fn test_all_fields_non_empty_and_valid() {
    let info = Count {
        id: 1,
        debtor: String::from("John Doe"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 1,
        installments: 3,
        value: 100.0,
        status: true,
        nature: String::from("Casa"),
    };

    let result = info.is_empty();
    assert_eq!(result, false);
}

// Returns false if debtor field is empty or contains non-alphabetic characters.
// #[tokio::test]
async fn test_debtor_field_empty_or_non_alphabetic() {
    let info = Count {
        id: 1,
        debtor: String::from(""),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 1,
        installments: 3,
        value: 100.0,
        status: true,
        nature: String::from("Casa"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if title field is empty.
// #[tokio::test]
async fn test_title_field_empty() {
    let info = Count {
        id: 0,
        debtor: String::from("John Doe"),
        title: String::from(""),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        installments: 1,
        value: 100.0,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if value field is zero.
// #[tokio::test]
async fn test_value_field_zero() {
    let info = Count {
        id: 0,
        debtor: String::from("John Doe"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        installments: 1,
        value: 0.0,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if installments field is zero.
// #[tokio::test]
async fn test_installments_field_zero() {
    let info = Count {
        id: 0,
        debtor: String::from("John Doe"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        installments: 0,
        value: 100.0,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if debtor field contains only spaces.
// #[tokio::test]
async fn test_debtor_field_only_spaces() {
    let info = Count {
        id: 1,
        debtor: String::from("     "),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 100.0,
        installments: 2,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if debtor field contains only non-alphabetic characters.
// #[tokio::test]
async fn test_debtor_field_only_non_alphabetic() {
    let info = Count {
        id: 1,
        debtor: String::from("12345"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 100.0,
        installments: 2,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns true if debtor field contains alphabetic characters and spaces.
// #[tokio::test]
async fn test_debtor_field_alphabetic_and_spaces() {
    let info = Count {
        id: 1,
        debtor: String::from("John Doe     "),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 100.0,
        installments: 2,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns true if value field is non-zero.
// #[tokio::test]
async fn test_value_field_non_zero() {
    let info = Count {
        id: 1,
        debtor: String::from("John Doe"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 100.0,
        installments: 2,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns true if installments field is non-zero.
// #[tokio::test]
async fn test_installments_field_non_zero() {
    let info = Count {
        id: 1,
        debtor: String::from("John Doe"),
        title: String::from("Invoice"),
        description: String::from("Payment for services"),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 100.0,
        installments: 2,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if Count struct is empty.
// #[tokio::test]
async fn test_info_struct_empty() {
    let info = Count {
        id: 0,
        debtor: String::from(""),
        title: String::from(""),
        description: String::from(""),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 0.0,
        installments: 0,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}

// Returns false if Count struct has only debtor field.
// #[tokio::test]
async fn test_info_struct_only_debtor_field() {
    let info = Count {
        id: 0,
        debtor: String::from("John Doe"),
        title: String::from(""),
        description: String::from(""),
        date_in: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        date_out: NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        paid_installments: 0,
        value: 0.0,
        installments: 0,
        status: false,
        nature: String::from("Investimentos"),
    };

    let result = info.is_empty();
    assert_eq!(result, true);
}
