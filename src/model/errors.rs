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

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum DataBaseError {
    CreatePoolError(ErrorLog<'static>),
    AddUserError(ErrorLog<'static>),
    AddCountError(ErrorLog<'static>),
    AddPeopleError(ErrorLog<'static>),
    GetUserError(ErrorLog<'static>),
    GetConfigError(ErrorLog<'static>),
    GetCountsError(ErrorLog<'static>),
    GetPeopleError(ErrorLog<'static>),
    EditPeopleError(ErrorLog<'static>),
    EditCountsError(ErrorLog<'static>),
    EditUserError(ErrorLog<'static>),
    DeletePeopleError(ErrorLog<'static>),
    DeleteUserError(ErrorLog<'static>),
    DeleteCountError(ErrorLog<'static>),
    DataTypeInvalid(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ControlError {
    ErrorExternDB(DataBaseError),
    ErrorExtern(ErrorLog<'static>),
    ErrorToAddUser(ErrorLog<'static>),
    UserNotExists(ErrorLog<'static>),
    ErrorAuthenticate(ErrorLog<'static>),
    ErrorValueInvalid(ErrorLog<'static>),
    ErrorNotSave(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ApiError {
    ErrorMailjet(ErrorLog<'static>),
    ErrorRouter(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum PeopleError {
    CPFInvalid(ErrorLog<'static>),
}
