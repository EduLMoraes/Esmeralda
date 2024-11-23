use chrono::NaiveDate;

#[allow(unused)]
pub struct People {
    id: String,
    name: String,
    wage: f32,
    cell_phone: String,
    birthday: NaiveDate,
    rg: String,
    cpf: String,
    surname: String,
}

#[allow(unused)]
impl People {
    pub fn from(
        id: String,
        name: String,
        wage: f32,
        cell_phone: String,
        birthday: NaiveDate,
        rg: String,
        cpf: String,
        surname: String,
    ) -> People {
        People {
            id: id,
            name: name,
            wage: wage,
            cell_phone: cell_phone,
            birthday: birthday,
            rg: rg,
            cpf: cpf,
            surname: surname,
        }
    }

    pub fn valid_cpf(cpf: &str) -> bool {
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
