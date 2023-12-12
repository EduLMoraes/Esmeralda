use super::errors::ErrorLog;
use super::structs::*;
use super::db::*;
use super::Error;
use super::crypt::crpt;

pub async fn login(mut user: User) -> Result<(), ControlError>{
    let db = get_database_instance();

    user.password = crpt(user.password);


    let data_user = Data::User(user.clone());

    let db_user = db.get(data_user).await.map_err(|err| {
        ControlError::ErrorExtern(err)
    })?;

    match db_user {
        Data::User(data) => {
            if data.password == user.password{
                Ok(())
            }else {
                Err(ControlError::ErrorAuthenticate(
                    ErrorLog { title: "Password incorrect", code: 305, file: "controller.rs" }
                ))
            }
        },
        _ => Err(ControlError::ErrorAuthenticate(
            ErrorLog { title: "Data type received is invalid", code: 306, file: "controller.rs" }
        ))
    }
}
 
pub async fn add_user(new_user: NewUser, password: String) -> Result<(), ControlError> {
    if new_user.password == password {
        let db = get_database_instance();

        let new_user: Data = Data::NewUser(new_user);

        db.add(new_user).await.map_err(|err| {
            ControlError::ErrorExtern(err)
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
    ErrorExtern(DataBaseError),

    #[error("Add user error")]
    ErrorToAddUser(ErrorLog<'static>),

    #[error("Authenticate error")]
    ErrorAuthenticate(ErrorLog<'static>),

    #[error("Error of value invalid")]
    ErrorValueInvalid(ErrorLog<'static>)
}
