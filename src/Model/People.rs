use chrono::NaiveDate;

use super::errors::{ErrorLog, PeopleError};

#[derive(PartialOrd, Debug, Clone)]
pub struct People {
    pub id: String,
    pub addres: String,
    pub name: String,
    pub wage: f32,
    pub cell_phone: String,
    pub birthday: NaiveDate,
    pub rg: String,
    pub cpf: String,
    pub surname: String,
    pub voter_registration: String,
    pub provider: String,
}

#[allow(unused)]
impl People {
    pub fn new(name: &String) -> People {
        People {
            id: Default::default(),
            name: name.clone(),
            wage: Default::default(),
            cell_phone: String::new(),
            birthday: NaiveDate::default(),
            rg: Default::default(),
            cpf: Default::default(),
            surname: Default::default(),
            voter_registration: Default::default(),
            provider: Default::default(),
            addres: Default::default(),
        }
    }

    pub fn from(
        id: String,
        name: String,
        wage: f32,
        cell_phone: String,
        birthday: NaiveDate,
        rg: String,
        cpf: String,
        surname: String,
        voter_registration: String,
        provider: String,
        address: String,
    ) -> Result<Self, PeopleError> {
        if People::validate_cpf(&cpf) || cpf.is_empty() {
            return Ok(People {
                id: id,
                name: name,
                wage: wage,
                cell_phone: cell_phone,
                birthday: birthday,
                rg: rg,
                cpf: cpf,
                surname: surname,
                voter_registration: voter_registration,
                provider: provider,
                addres: address,
            });
        }
        Err(PeopleError::CPFInvalid(ErrorLog {
            title: "This cpf is invalid!",
            code: 902,
            file: "People.rs",
        }))
    }

    pub fn validate_cpf(cpf: &str) -> bool {
        let cpf_regex: Regex = regex::Regex::new(r"^\d{3}\.\d{3}\.\d{3}-\d{2}").unwrap();

        if !cpf_regex.is_match(cpf) {
            return false;
        }

        let mut dig_valid: u32 = 0;
        let mut cpf: Vec<char> = cpf.chars().collect();
        let _ = cpf.remove(3);
        let _ = cpf.remove(6);
        let _ = cpf.remove(9);

        if cpf == vec!['0'; 11] {
            return false;
        }

        use regex::Regex;

        let mut multply: i32 = 10;
        for letter in 0..(cpf.len() - 2) {
            dig_valid += cpf[letter].to_digit(10).unwrap() * multply as u32;
            multply -= 1;
        }

        dig_valid %= 11;

        if dig_valid < 2 {
            dig_valid = 0;
        } else {
            dig_valid = 11 - dig_valid;
        }

        if dig_valid != cpf[9].to_digit(10).unwrap() {
            return false;
        }

        let mut multply: i32 = 11;
        dig_valid = 0;
        for letter in 0..(cpf.len() - 1) {
            dig_valid += cpf[letter].to_digit(10).unwrap() * multply as u32;
            multply -= 1;
        }

        dig_valid %= 11;

        if dig_valid < 2 {
            dig_valid = 0;
        } else {
            dig_valid = 11 - dig_valid;
        }

        if dig_valid != cpf[10].to_digit(10).unwrap() {
            return false;
        }

        true
    }
}

impl PartialEq for People {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.wage == other.wage
            && self.cell_phone == other.cell_phone
            && self.birthday == other.birthday
            && self.rg == other.rg
            && self.surname == other.surname
            && self.cpf == other.cpf
            && self.voter_registration == other.voter_registration
            && self.provider == other.provider
    }
}
