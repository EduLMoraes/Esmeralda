use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserControlError {
    #[error("Invalid ID: {0}")]
    Id(String),
    #[error("Invalid Name: {0}")]
    Name(String),
    #[error("Invalid RG: {0}")]
    Rg(String),
    #[error("Invalid CPF: {0}")]
    Cpf(String),
    #[error("Invalid UI Wage: {0}")]
    UiWage(String),
    #[error("Invalid Cell Phone: {0}")]
    CellPhone(String),
    #[error("Invalid Date of Birth: {0}")]
    DateOfBirth(String),
    #[error("Invalid Voter Registration: {0}")]
    VoterRegistration(String),
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct People {
    id: String,
    rg: Option<String>,
    cpf: Option<String>,
    name: String,
    surname: Option<String>,
    ui_wage: Option<f64>,
    cell_phone: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_naive_date",
        deserialize_with = "crate::deserialize_naive_date"
    )]
    date_of_birth: NaiveDate,
    voter_registration: Option<String>,
}

impl People {
    pub fn new(
        rg: Option<String>,
        cpf: Option<String>,
        name: String,
        surname: Option<String>,
        ui_wage: Option<f64>,
        cell_phone: Option<String>,
        date_of_birth: NaiveDate,
        voter_registration: Option<String>,
    ) -> Result<Self, UserControlError> {
        let people = People {
            id: uuid::Uuid::new_v4().to_string(),
            rg,
            cpf,
            name,
            surname,
            ui_wage,
            cell_phone,
            date_of_birth,
            voter_registration,
        };
        people.validate()?;
        Ok(people)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn rg(&self) -> Option<&String> {
        self.rg.as_ref()
    }

    pub fn cpf(&self) -> Option<&String> {
        self.cpf.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn surname(&self) -> Option<&String> {
        self.surname.as_ref()
    }

    pub fn ui_wage(&self) -> Option<f64> {
        self.ui_wage
    }

    pub fn cell_phone(&self) -> Option<&String> {
        self.cell_phone.as_ref()
    }

    pub fn date_of_birth(&self) -> &NaiveDate {
        &self.date_of_birth
    }

    pub fn voter_registration(&self) -> Option<&String> {
        self.voter_registration.as_ref()
    }

    pub fn set_rg(&mut self, rg: String) -> Result<(), UserControlError> {
        if rg.len() > 8 {
            return Err(UserControlError::Rg("Invalid RG".to_string()));
        }
        self.rg = Some(rg);
        Ok(())
    }

    pub fn set_cpf(&mut self, cpf: String) -> Result<(), UserControlError> {
        if cpf.len() > 11 {
            return Err(UserControlError::Cpf("Invalid CPF".to_string()));
        }
        self.cpf = Some(cpf);
        Ok(())
    }

    pub fn set_name(&mut self, name: String) -> Result<(), UserControlError> {
        if name.is_empty() {
            return Err(UserControlError::Name("Name cannot be empty".to_string()));
        }
        self.name = name;
        Ok(())
    }

    pub fn set_surname(&mut self, surname: String) -> Result<(), UserControlError> {
        self.surname = Some(surname);
        Ok(())
    }

    pub fn set_ui_wage(&mut self, ui_wage: f64) -> Result<(), UserControlError> {
        if ui_wage < 0.0 {
            return Err(UserControlError::UiWage(
                "UI wage cannot be negative".to_string(),
            ));
        }
        self.ui_wage = Some(ui_wage);
        Ok(())
    }

    pub fn set_cell_phone(&mut self, cell_phone: String) -> Result<(), UserControlError> {
        if cell_phone.len() > 11 {
            return Err(UserControlError::CellPhone(
                "Invalid cell phone".to_string(),
            ));
        }
        self.cell_phone = Some(cell_phone);
        Ok(())
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: NaiveDate) -> Result<(), UserControlError> {
        self.date_of_birth = date_of_birth;
        Ok(())
    }

    pub fn set_voter_registration(
        &mut self,
        voter_registration: String,
    ) -> Result<(), UserControlError> {
        if voter_registration.len() > 12 {
            return Err(UserControlError::VoterRegistration(
                "Invalid voter registration".to_string(),
            ));
        }
        self.voter_registration = Some(voter_registration);
        Ok(())
    }

    fn validate(&self) -> Result<(), UserControlError> {
        if self.id.is_empty() {
            return Err(UserControlError::Id("Id cannot be empty".to_string()));
        }
        if self.name.is_empty() {
            return Err(UserControlError::Name("Name cannot be empty".to_string()));
        }
        if let Some(rg) = &self.rg {
            if rg.len() > 8 {
                return Err(UserControlError::Rg("Invalid RG".to_string()));
            }
        }
        if let Some(cpf) = &self.cpf {
            if cpf.len() > 11 {
                return Err(UserControlError::Cpf("Invalid CPF".to_string()));
            }
        }
        if let Some(ui_wage) = self.ui_wage {
            if ui_wage < 0.0 {
                return Err(UserControlError::UiWage(
                    "UI wage cannot be negative".to_string(),
                ));
            }
        }
        if let Some(cell_phone) = &self.cell_phone {
            if cell_phone.len() > 11 {
                return Err(UserControlError::CellPhone(
                    "Invalid cell phone".to_string(),
                ));
            }
        }
        if let Some(voter_registration) = &self.voter_registration {
            if voter_registration.len() > 12 {
                return Err(UserControlError::VoterRegistration(
                    "Invalid voter registration".to_string(),
                ));
            }
        }
        Ok(())
    }
}
