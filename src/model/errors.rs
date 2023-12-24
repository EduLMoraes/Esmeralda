use std::fmt;

/// ErrorLog will standardize our program's error output,
/// to facilitate the location of the defect and the problem we
/// we use the title for a brief description of the error, the code for
/// identify which error is giving and the file where the error occurs
/// was issued.
///
/// The error codes are as follows:
/// - 816: Invalid data type for the database.
/// - 812: Error of non-existent table in the database.
/// - 808: Error inserting data into the database.
/// - 804: Error searching for the database and not finding it.
/// - 802: Error configuring the database.
/// - 800: Error in another file that started in the database.
/// - 500: Error in third-party library.
/// - 404: Page not found error.
/// - 306: Error inputting data into the function, invalid type.
/// - 305: Error when entering data into the function, invalid or incomplete data.
/// - 304: Error when searching for a module and not finding it.

/// Represents an error log with a title, code, and file.
#[derive(Debug, PartialEq)]
pub struct ErrorLog<'a> {
    pub title: &'a str,
    pub code: i32,
    pub file: &'a str,
}

impl<'a> fmt::Display for ErrorLog<'a> {
    /// Formats the error log as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the formatted string to.
    ///
    /// # Returns
    ///
    /// Returns a `fmt::Result` indicating success or failure.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error: {}\n -> Code: {}\n -> File: {}\n",
            self.title, self.code, self.file
        )
    }
}

use crate::Error;
/// Represents different types of errors that can occur in a database.
#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DataBaseError {
    /// Config error
    #[error("Error to get config")]
    GetConfigError(ErrorLog<'static>),

    /// Require pool error
    #[error("Error to create pool")]
    CreatePoolError(ErrorLog<'static>),

    /// Add user not working
    #[error("Error to add user")]
    AddUserError(ErrorLog<'static>),

    /// Config error
    #[error("Error to get user from db")]
    GetUserError(ErrorLog<'static>),

    /// DataType not Accept
    #[error("Error in data type receive")]
    DataTypeInvalid(ErrorLog<'static>),
}

/// Represents different types of control errors that can occur.
#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    /// Error related to an external module.
    #[error("Error of module extern")]
    ErrorExternDB(DataBaseError),

    /// Error when adding a user.
    #[error("Add user error")]
    ErrorExtern(ErrorLog<'static>),

    /// Error when adding a user.
    #[error("Add user error")]
    ErrorToAddUser(ErrorLog<'static>),

    /// Authentication error.
    #[error("Authenticate error")]
    ErrorAuthenticate(ErrorLog<'static>),

    /// Error when a value is invalid.
    #[error("Error of value invalid")]
    ErrorValueInvalid(ErrorLog<'static>),

    /// Error when saving data.
    #[error("Error to save")]
    ErrorNotSave(ErrorLog<'static>)
}