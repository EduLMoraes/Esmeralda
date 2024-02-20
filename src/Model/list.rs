use crate::prelude::model::Info::Info;
use crate::prelude::model::Debtor::Debtor;
use std::collections::HashMap;
use std::cmp::Reverse;


/// Represents a collection of `Info` objects and provides methods to manipulate and order the list based on different criteria.
///
/// Example Usage:
/// ```
/// let mut interface = InterfaceInfo::new();
///
/// let info1 = Info { id: 1, debtor: "John", value: 100.0 };
/// let info2 = Info { id: 2, debtor: "Alice", value: 200.0 };
///
/// interface.put(info1);
/// interface.put(info2);
///
/// let ordered_list = interface.order_by_id(true);
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
/// - The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods return a new `InterfaceInfo` object with the ordered list.
/// - The `order_alphabetical` method returns a new `InterfaceInfo` object with the alphabetically ordered list.
/// - The `test` method returns a randomly generated `InterfaceInfo` object for testing purposes.
/// - The `Display` trait implementation allows printing the list of `Info` objects.
///
/// Flow:
/// 1. The `InterfaceInfo` struct has a `list` field that stores a vector of `Info` objects.
/// 2. The `new` method initializes an empty `InterfaceInfo` object.
/// 3. The `len` method returns the length of the list.
/// 4. The `put` method inserts a `Info` object at the beginning of the list.
/// 5. The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods order the list based on the specified criteria.
/// 6. The `order_alphabetical` method orders the list alphabetically based on the specified column.
/// 7. The `test` method generates a random list of `Info` objects for testing purposes.
/// 8. The `Display` trait implementation allows printing the list of `Info` objects.

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceInfo {
    pub list: Vec<Info>,
}

impl InterfaceInfo {
    #[allow(dead_code)]
    pub fn new() -> InterfaceInfo {
        InterfaceInfo { list: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn put(&mut self, value: Info) {
        self.list.insert(0, value)
    }

    pub fn order_by_id(&self, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();

        if crescent {
            list.list.sort_by_cached_key(|a| a.id);
        } else {
            list.list.sort_by_cached_key(|a| Reverse(a.id));
        }

        list
    }

    pub fn order_by_value(&self, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();
        let width: usize = list.len();

        loop {
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width {
                    if list.list[i].value < list.list[i - 1].value {
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            } else {
                for i in 1..width {
                    if list.list[i].value > list.list[i - 1].value {
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }

            if comparisions {
                break;
            }
        }
        list
    }

    pub fn order_by_status(&self, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();

        if crescent {
            list.list.sort_by_cached_key(|a| a.status);
        } else {
            list.list.sort_by_cached_key(|a| Reverse(a.status));
        }

        list
    }

    pub fn order_by_date(&self, is_in: bool, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();

        if crescent {
            list.list
                .sort_by_cached_key(|a| if is_in { a.date_in } else { a.date_out });
        } else {
            list.list
                .sort_by_cached_key(|a| Reverse(if is_in { a.date_in } else { a.date_out }));
        }

        list
    }

    pub fn order_by_installments(&self, is_paid: bool, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();

        if crescent {
            list.list.sort_by_cached_key(|a| {
                if is_paid {
                    a.paid_installments
                } else {
                    a.installments
                }
            });
        } else {
            list.list.sort_by_cached_key(|a| {
                Reverse(if is_paid {
                    a.paid_installments
                } else {
                    a.installments
                })
            });
        }

        list
    }

    pub fn order_alphabetical(&self, column: &str, crescent: bool) -> InterfaceInfo {
        let mut list = self.clone();

        if column == "name" {
            if crescent {
                list.list
                    .sort_by_cached_key(|a| a.debtor.to_string().to_lowercase());
            } else {
                list.list
                    .sort_by_cached_key(|a| Reverse(a.debtor.to_string().to_lowercase()));
            }
        } else if column == "title" {
            if crescent {
                list.list
                    .sort_by_cached_key(|a| a.title.to_string().to_lowercase());
            } else {
                list.list
                    .sort_by_cached_key(|a| Reverse(a.title.to_string().to_lowercase()));
            }
        } else if column == "description" {
            if crescent {
                list.list
                    .sort_by_cached_key(|a| a.description.to_string().to_lowercase());
            } else {
                list.list
                    .sort_by_cached_key(|a| Reverse(a.description.to_string().to_lowercase()));
            }
        } else if column == "nature" {
            if crescent {
                list.list
                    .sort_by_cached_key(|a| a.nature.to_string().to_lowercase());
            } else {
                list.list
                    .sort_by_cached_key(|a| Reverse(a.nature.to_string().to_lowercase()));
            }
        }

        list
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
        if item.is_empty(){
            return self.list.clone();
        }

        let mut itens: Vec<Info> = Vec::new();

        for info in &self.list{
            if item == info.debtor 
            || item.to_lowercase() == info.nature.to_lowercase()
            || item.to_lowercase() == info.title.to_lowercase()
            || item.to_lowercase() == info.description.to_lowercase()
            || item == info.date_in.to_string()
            || item == info.date_out.to_string()
            || item == info.id.to_string(){ 
                itens.push(
                    info.clone()
                );    
            }
        }

        return itens;
    }
}

use std::fmt;
impl fmt::Display for InterfaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{:?}\n", self.list[i])?;
        }

        Ok(())
    }
}
