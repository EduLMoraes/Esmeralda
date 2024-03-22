use crate::prelude::model::Debtor::Debtor;
use crate::prelude::model::Info::Info;
use std::cmp::Reverse;
use std::collections::HashMap;

/// Represents a collection of `Info` objects and provides methods to manipulate and order the list based on different criteria.
///
/// Example Usage:
/// ```
/// let mut List = ListInfo::new();
///
/// let info1 = Info { id: 1, debtor: "John", value: 100.0 };
/// let info2 = Info { id: 2, debtor: "Alice", value: 200.0 };
///
/// List.put(info1);
/// List.put(info2);
///
/// let ordered_list = List.order_by_id(true);
///
/// println!("{}", ordered_list);
/// ```
///
/// Outputs:
/// ```
/// Info { id: 1, debtor: "John", value: 100.0 }
/// Info { id: 2, debtor: "Alice", value: 200.0 }
/// ```
///
/// Inputs:
/// - `value`: A `Info` object to be inserted into the list.
/// - `crescent`: A boolean value indicating whether the list should be ordered in ascending or descending order.
/// - `column`: A string indicating the column to be used for alphabetical ordering.
///
/// Outputs:
/// - The `len` method returns the number of `Info` objects in the list.
/// - The `put` method inserts a `Info` object into the list.
/// - The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods return a new `ListInfo` object with the ordered list.
/// - The `order_alphabetical` method returns a new `ListInfo` object with the alphabetically ordered list.
/// - The `test` method returns a randomly generated `ListInfo` object for testing purposes.
/// - The `Display` trait implementation allows printing the list of `Info` objects.
///
/// Flow:
/// 1. The `ListInfo` struct has a `list` field that stores a vector of `Info` objects.
/// 2. The `new` method initializes an empty `ListInfo` object.
/// 3. The `len` method returns the length of the list.
/// 4. The `put` method inserts a `Info` object at the beginning of the list.
/// 5. The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods order the list based on the specified criteria.
/// 6. The `order_alphabetical` method orders the list alphabetically based on the specified column.
/// 7. The `test` method generates a random list of `Info` objects for testing purposes.
/// 8. The `Display` trait implementation allows printing the list of `Info` objects.

#[derive(Clone, Debug, PartialEq)]
pub struct ListInfo {
    pub list: Vec<Info>,
}

impl ListInfo {
    #[allow(dead_code)]
    pub fn new() -> ListInfo {
        ListInfo { list: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn put(&mut self, value: Info) {
        self.list.insert(0, value)
    }

    pub fn get_total(&self) -> f32 {
        let mut sum = 0.0;

        for i in &self.list {
            sum += i.value;
        }

        sum
    }

    pub fn order_by_id(&mut self, crescent: bool) {
        if crescent {
            self.list.sort_by_cached_key(|a| a.id);
        } else {
            self.list.sort_by_cached_key(|a| Reverse(a.id));
        }
    }

    pub fn order_by_value(&mut self, crescent: bool) {
        let width = self.len();
        loop {
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width {
                    if self.list[i].value < self.list[i - 1].value {
                        comparisions = false;

                        let tmp = self.list[i].clone();

                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            } else {
                for i in 1..width {
                    if self.list[i].value > self.list[i - 1].value {
                        comparisions = false;

                        let tmp = self.list[i].clone();

                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }

            if comparisions {
                break;
            }
        }
    }

    pub fn order_by_status(&mut self, crescent: bool) {
        if crescent {
            self.list.sort_by_cached_key(|a| a.status);
        } else {
            self.list.sort_by_cached_key(|a| Reverse(a.status));
        }
    }

    pub fn order_by_date(&mut self, is_in: bool, crescent: bool) {
        if crescent {
            self.list
                .sort_by_cached_key(|a| if is_in { a.date_in } else { a.date_out });
        } else {
            self.list
                .sort_by_cached_key(|a| Reverse(if is_in { a.date_in } else { a.date_out }));
        }
    }

    pub fn order_by_installments(&mut self, is_paid: bool, crescent: bool) {
        if crescent {
            self.list.sort_by_cached_key(|a| {
                if is_paid {
                    a.paid_installments
                } else {
                    a.installments
                }
            });
        } else {
            self.list.sort_by_cached_key(|a| {
                Reverse(if is_paid {
                    a.paid_installments
                } else {
                    a.installments
                })
            });
        }
    }

    pub fn order_alphabetical(&mut self, column: &str, crescent: bool) {
        match column {
            "name" => {
                if crescent {
                    self.list
                        .sort_by_cached_key(|a| a.debtor.to_string().to_lowercase());
                } else {
                    self.list
                        .sort_by_cached_key(|a| Reverse(a.debtor.to_string().to_lowercase()));
                }
            }
            "title" => {
                if crescent {
                    self.list
                        .sort_by_cached_key(|a| a.title.to_string().to_lowercase());
                } else {
                    self.list
                        .sort_by_cached_key(|a| Reverse(a.title.to_string().to_lowercase()));
                }
            }
            "description" => {
                if crescent {
                    self.list
                        .sort_by_cached_key(|a| a.description.to_string().to_lowercase());
                } else {
                    self.list
                        .sort_by_cached_key(|a| Reverse(a.description.to_string().to_lowercase()));
                }
            }
            "nature" => {
                if crescent {
                    self.list
                        .sort_by_cached_key(|a| a.nature.to_string().to_lowercase());
                } else {
                    self.list
                        .sort_by_cached_key(|a| Reverse(a.nature.to_string().to_lowercase()));
                }
            }
            &_ => todo!(),
        }
    }

    #[allow(unused_assignments)]
    pub fn filter_debtors(&self) -> Vec<Debtor> {
        let mut debtors_map: HashMap<String, Debtor> = HashMap::new();

        for info in &self.list {
            let name = info.debtor.trim().to_string();
            let debtor = debtors_map.entry(name.clone()).or_insert(Debtor::new(
                info.id,
                name.clone().trim(),
                0.0,
                0.0,
            ));
            let mut value = 0.0;

            if info.installments != info.paid_installments {
                let remaining_installments = info.installments - info.paid_installments;

                value = info.value * remaining_installments as f32;
            } else {
                value = info.value * info.paid_installments as f32;
            }

            if info.status {
                debtor.add_value(value);
            } else {
                debtor.add_value(info.value * info.paid_installments as f32);
                debtor.add_debt(value);
            }
        }

        let mut debtors: Vec<Debtor> = debtors_map.into_values().collect();
        debtors.sort_by_key(|debtor| debtor.get_id());

        debtors
    }

    pub fn search(&self, item: String) -> Vec<Info> {
        self.list
            .iter()
            .filter(|count| {
                count.debtor == item
                    || item.to_lowercase() == count.nature.to_lowercase()
                    || item.to_lowercase() == count.title.to_lowercase()
                    || item.to_lowercase() == count.description.to_lowercase()
                    || item == count.date_in.to_string()
                    || item == count.date_out.to_string()
                    || item == count.id.to_string()
            })
            .cloned()
            .collect::<Vec<Info>>()
    }
}

use std::fmt;
impl fmt::Display for ListInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{:?}\n", self.list[i])?;
        }

        Ok(())
    }
}
