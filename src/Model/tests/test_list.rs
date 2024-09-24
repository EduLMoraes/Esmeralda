#[allow(unused_imports)]
use super::Count::Count;
#[allow(unused_imports)]
use super::List::*;
#[allow(unused_imports)]
use super::NaiveDate;

#[cfg(test)]
mod tests_list {
    use super::*;

    #[test]
    fn test_filter_by_nature_correct() {
        let count_1 = Count {
            id: 1,
            debtor: "Debtor".to_string(),
            title: "Title".to_string(),
            description: "Description".to_string(),
            value: 100.0,
            paid_installments: 0,
            installments: 1,
            date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            status: true,
            nature: "Nature".to_string(),
        };
        let count_2 = Count {
            id: 2,
            debtor: "Debtor".to_string(),
            title: "Title".to_string(),
            description: "Description".to_string(),
            value: 100.0,
            paid_installments: 0,
            installments: 1,
            date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            status: true,
            nature: "Food".to_string(),
        };

        let list_count = ListCount {
            list: vec![count_1.clone(), count_2.clone()],
            years: vec![2022],
        };

        assert_eq!(
            list_count.filter_by_nature(&String::from("Nature")),
            vec![count_1]
        );
        assert_eq!(
            list_count.filter_by_nature(&String::from("Food")),
            vec![count_2]
        );
    }
}
