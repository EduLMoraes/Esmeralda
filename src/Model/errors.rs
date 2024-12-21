use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ErrorLog<'a> {
    pub title: &'a str,
    pub code: i32,
    pub file: &'a str,
}

impl<'a> fmt::Display for ErrorLog<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error: {}\n -> Code: {}\n -> File: {}\n",
            self.title, self.code, self.file
        )
    }
}

use crate::prelude::Error;
#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DataBaseError {
    #[error("Error to get config")]
    GetConfigError(ErrorLog<'static>),

    #[error("Error to create pool")]
    CreatePoolError(ErrorLog<'static>),

    #[error("Error to add user")]
    AddUserError(ErrorLog<'static>),

    #[error("Error to add user")]
    AddCountError(ErrorLog<'static>),

    #[error("Error to get user from db")]
    GetUserError(ErrorLog<'static>),

    #[error("Error to get user from db")]
    GetCountsError(ErrorLog<'static>),

    #[error("Error to get user from db")]
    EditCountsError(ErrorLog<'static>),

    #[error("Error to get user from db")]
    EditUserError(ErrorLog<'static>),

    #[error("Error to delete user from db")]
    DeleteUserError(ErrorLog<'static>),

    #[error("Errror to delete count from db")]
    DeleteCountError(ErrorLog<'static>),

    #[error("Error in data type receive")]
    DataTypeInvalid(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    #[error("Error of module extern")]
    ErrorExternDB(DataBaseError),

    #[error("Add user error")]
    ErrorExtern(ErrorLog<'static>),

    #[error("Add user error")]
    ErrorToAddUser(ErrorLog<'static>),

    #[error("Error of user trying loggin not exists")]
    UserNotExists(ErrorLog<'static>),

    #[error("Authenticate error")]
    ErrorAuthenticate(ErrorLog<'static>),

    #[error("Error of value invalid")]
    ErrorValueInvalid(ErrorLog<'static>),

    #[error("Error to save")]
    ErrorNotSave(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("Error to send email")]
    ErrorMailjet(ErrorLog<'static>),

    #[error("Error on router")]
    ErrorRouter(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum PeopleError {
    #[error("CPF Invalid")]
    CPFInvalid(ErrorLog<'static>),
}
