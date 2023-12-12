use super::errors::ErrorLog;
use super::structs::*;
use super::db::*;
use super::Error;
use super::crypt::crpt;

pub async fn login(mut user: User) -> Result<(), ControlError>{
    let db = DataBase::new().map_err(|_| {
        ControlError::ErrorExtern(ErrorLog {
            title: "Error to get pool by DataBase",
            code: 800,
            file: "controller.rs",
        })
    })?;

    user.password = crpt(user.password);

    let user = Data::User(user);
    let db_user = db.get(user);

    Ok(())
}

enum ErrorLogin{
    
}

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

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    #[error("Error of module extern")]
    ErrorExtern(ErrorLog<'static>),

    #[error("Add user error")]
    ErrorToAddUser(ErrorLog<'static>),

    #[error("Authenticate error")]
    ErrorAuthenticate(ErrorLog<'static>),

    #[error("Error of value invalid")]
    ErrorValueInvalid(ErrorLog<'static>)
}
