use chrono::Datelike;

use crate::control::delete;
use crate::prelude::model::Count::Count;
use crate::prelude::model::Debtor::Debtor;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ListCount {
    pub list: Vec<Count>,
    pub years: Vec<i16>,
}

impl ListCount {
    #[allow(dead_code)]
    pub fn new() -> ListCount {
        ListCount {
            list: Vec::new(),
            years: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn put(&mut self, mut value: Count) {
        self.order_by_id(false);

        if !self.list.is_empty() {
            value.id = self.list[0].id;
            value.new_id();
        }

        self.list.insert(0, value)
    }

    pub fn remove(&mut self, id_count: &i32) -> bool {
        let rnt = crate::tokio::runtime::Runtime::new().unwrap();

        if rnt.block_on(delete(id_count)).is_ok() {
            for i in 0..self.list.len() {
                if &self.list[i].id == id_count {
                    self.list.remove(i);
                    break;
                }
            }

            true
        } else {
            false
        }
    }

    pub fn get_total(&self) -> f32 {
        let mut sum = 0.0;
        for count in &self.list {
            sum += count.value * count.paid_installments as f32;
        }

        sum
    }

    pub fn get_total_debt(&self) -> f32 {
        let mut sum = 0.0;
        for count in &self.list {
            if !count.status {
                sum += count.value * count.installments as f32;
            }
        }

        sum
    }

    pub fn get_total_perfomance(&self) -> f32 {
        let mut perfomance = 0.0;

        for c in &self.list {
            if c.nature == *"Receita" {
                perfomance += c.value * c.paid_installments as f32;
            } else {
                perfomance -= c.value * c.installments as f32;
            }
        }
        perfomance
    }

    pub fn get_perfomance_months(&self) -> Vec<f32> {
        let mut months: Vec<f32> = vec![0.0; 12];

        for count in &self.list {
            if count.nature == *"Receita" {
                months[(count.date_in.month() - 1) as usize] += count.value;
            } else {
                months[(count.date_in.month() - 1) as usize] -= count.value;
            }
        }

        months
    }

    pub fn get_data_months(&self) -> Vec<(String, Vec<f32>)> {
        let mut data: Vec<(String, Vec<f32>)> = vec![];

        let mut list = self.clone();

        list.order_alphabetical("nature", true);

        let mut months: Vec<f32> = vec![0.0; 12];
        let mut cont = 0;

        for count in list.list {
            if !data.is_empty() && count.nature == data[cont].0 {
                self.group_data_months(&mut months, &count);
            } else {
                months = vec![0.0; 12];
                self.group_data_months(&mut months, &count);

                data.push((count.nature, months.clone()));

                if data.len() > 1 {
                    cont += 1;
                }
            }

            data[cont].1 = months.clone();
        }

        data
    }

    fn group_data_months(&self, months: &mut Vec<f32>, count: &Count) {
        if count.date_in.month0() <= count.date_out.month0()
            && count.date_in.year() == count.date_out.year()
        {
            for month in count.date_in.month0()..count.date_out.month0() {
                months[month as usize] += count.value;
            }
        } else if count.date_in.year() < count.date_out.year() {
            for month in count.date_in.month0()..12 {
                months[month as usize] += count.value;
            }
        }
    }

    pub fn order_by_id(&mut self, crescent: bool) {
        if crescent {
            self.list.sort_unstable_by_key(|a| a.id);
        } else {
            self.list.sort_unstable_by_key(|a| Reverse(a.id));
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

        for Count in &self.list {
            let name = Count.debtor.trim().to_string();
            let debtor = debtors_map.entry(name.clone()).or_insert(Debtor::new(
                Count.id,
                name.clone().trim(),
                0.0,
                0.0,
                0.0,
            ));

            if Count.nature == "Receita" {
                debtor.add_receipt(Count.value * Count.paid_installments as f32);
            } else {
                let mut value = 0.0;

                if Count.installments != Count.paid_installments {
                    let remaining_installments = Count.installments - Count.paid_installments;

                    value = Count.value * remaining_installments as f32;
                } else {
                    value = Count.value * Count.paid_installments as f32;
                }

                if Count.status {
                    debtor.add_value(value);
                } else {
                    debtor.add_value(Count.value * Count.paid_installments as f32);
                    debtor.add_debt(value);
                }
            }
        }

        let mut debtors: Vec<Debtor> = debtors_map.into_values().collect();
        debtors.sort_by_key(|debtor| debtor.get_id());

        debtors
    }

    pub fn filter_by_nature(&self, item: &String) -> Vec<Count> {
        self.list
            .iter()
            .filter(|count| item.to_lowercase() == count.nature.to_lowercase())
            .cloned()
            .collect::<Vec<Count>>()
    }

    pub fn search(&self, item: &String) -> Vec<Count> {
        use rust_fuzzy_search::fuzzy_compare;

        self.list
            .iter()
            .filter(|count| {
                fuzzy_compare(&item, &count.debtor) > 0.5
                    || fuzzy_compare(&item, &count.nature) > 0.5
                    || fuzzy_compare(&item.to_lowercase(), &count.title.to_lowercase()) > 0.5
                    || fuzzy_compare(&item.to_lowercase(), &count.description.to_lowercase()) > 0.5
                    || fuzzy_compare(&item, &count.date_in.to_string()) > 0.5
                    || fuzzy_compare(&item, &count.date_out.to_string()) > 0.5
                    || item == &count.id.to_string()
            })
            .cloned()
            .collect::<Vec<Count>>()
    }

    pub fn pay(&mut self, id: i32) {
        for count in &mut self.list {
            if count.id == id {
                count.pay()
            }
        }
    }
}

use crate::std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    static ref GLOBAL_COUNTS: Mutex<ListCount> = Mutex::new(ListCount::new());
}

pub fn get_counts_instance() -> std::sync::MutexGuard<'static, ListCount> {
    GLOBAL_COUNTS.lock().unwrap()
}

use std::fmt;
impl fmt::Display for ListCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            writeln!(f, "{:?}", self.list[i])?;
        }

        Ok(())
    }
}
