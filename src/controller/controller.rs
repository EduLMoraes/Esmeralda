use super::db::*;
use super::errors::ErrorLog;
use super::structs::NewUser;
use super::Error;

pub async fn add_user(new_user: NewUser, password: String) -> Result<(), ControlError> {
    if new_user.password == password {
        let db = DataBase::new().map_err(|_| {
            ControlError::ErrorToAddUser(ErrorLog {
                title: "Error to get pool by DataBase",
                code: 800,
                file: "controller.rs",
            })
        })?;

        let new_user: Data = Data::NewUser(new_user);

        db.add(new_user).await.map_err(|_| {
            ControlError::ErrorToAddUser(ErrorLog {
                title: "Error to add user by DataBase",
                code: 800,
                file: "controller.rs",
            })
        })?;

        Ok(())
    } else {
        Err(ControlError::ErrorToAddUser(ErrorLog {
            title: "Password is not equal",
            code: 305,
            file: "controller.rs",
        }))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    #[error("Config error")]
    ErrorToAddUser(ErrorLog<'static>),
}
