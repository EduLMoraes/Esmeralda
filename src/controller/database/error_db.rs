use crate::structs_db::*;
use super::Error;
use crate::errors::ErrorLog;
use crate::structs::InterfaceInfo;


#[allow(dead_code)]
pub enum Data {
    NewUser(NewUser),
    User(User),
    UserDb(UserDb),
    Counts(InterfaceInfo, UserDb),
}

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DataBaseError {
    #[error("Config error")]
    GetConfigError(ErrorLog<'static>),

    #[error("Require pool error")]
    CreatePoolError(ErrorLog<'static>),

    #[error("Add user not working")]
    AddUserError(ErrorLog<'static>),

    #[error("Config error")]
    GetUserError(ErrorLog<'static>),

    #[error("DataType not Acept")]
    DataTypeInvalid(ErrorLog<'static>),
}
