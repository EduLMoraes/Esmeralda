use crate::prelude::chrono::{Datelike, NaiveDate};
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Count {
    pub id: i32,
    pub debtor: String,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub date_in: NaiveDate,
    pub date_out: NaiveDate,
    pub paid_installments: u32,
    pub installments: u32,
    pub status: bool,
    pub nature: String,
}

impl Count {
    pub fn from(
        name: &str,
        title: &str,
        desc: &str,
        value: f32,
        date_in: NaiveDate,
        installments: u32,
        nature: &str,
    ) -> Count {
        Count {
            id: 0,
            debtor: if !Regex::new(r"[^a-zA-Z\s]").unwrap().is_match(name) {
                String::from(name)
            } else {
                String::from("")
            },
            title: String::from(title),
            description: String::from(desc),
            value: value,
            date_in: date_in,
            date_out: {
                if nature != "Receita" {
                    let mut tmp_month = date_in.month() + installments;
                    let mut tmp_year = date_in.year();
                    let mut tmp_installments = tmp_month;

                    while tmp_month > 12 {
                        tmp_year += 1;
                        tmp_installments -= 12;
                        tmp_month = tmp_installments;
                    }

                    NaiveDate::from_ymd_opt(tmp_year, tmp_month, 1).unwrap()
                } else {
                    date_in
                }
            },
            paid_installments: 0,
            installments: installments,
            status: false,
            nature: String::from(nature),
        }
    }

    pub fn new_id(&mut self) {
        self.id = self.id + 1;
    }

    pub fn pay_all(&mut self) {
        self.paid_installments = self.installments;
        self.status = true;
    }

    pub fn pay(&mut self) {
        if self.paid_installments + 1 == self.installments {
            self.pay_all();
        } else if self.paid_installments < self.installments {
            self.paid_installments += 1;
        }
    }

    #[allow(unused)]
    pub fn to_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {:.2}, {}, {}, {}, {}, {}, {}",
            self.id,
            self.debtor,
            self.title,
            self.description,
            self.value,
            self.date_in,
            self.date_out,
            self.paid_installments,
            self.installments,
            self.nature,
            self.status
        )
    }

    pub fn is_empty(&self) -> bool {
        self.debtor.trim().is_empty()
            || Regex::new(r"[0-9]").unwrap().is_match(self.debtor.trim())
            || self.title.trim().is_empty()
            || self.installments == 0
            || self.value == 0.0
    }
}
