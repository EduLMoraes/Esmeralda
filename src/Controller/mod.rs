pub mod tests;
pub mod config;

use crate::prelude::chrono::{Datelike, NaiveDate};

use crate::prelude::model::{
    Database::*,
    errors::*,
    Info::Info,
    User::*,
    list::InterfaceInfo
};
use crate::prelude::utils::{
    export::csv::export_csv,
    export::html::export_html,
    export::pdf::export_pdf,
    validate::alphabetic::is_alphabetic,
    validate::email_valid
};
use crate::prelude::segurance::*;
use crate::prelude::Instant;
use crate::prelude::log;
use crate::prelude::env::var;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// Handles user authentication.
///
/// Retrieves a database instance, encrypts the user's password, and then checks if the password matches the one stored in the database.
/// If the password is correct, it generates a user instance and returns a success result.
/// Otherwise, it returns an error indicating an incorrect password or an invalid data type.
///
/// # Example
///
/// ```rust
/// let user = User {
///     username: "john_doe",
///     password: "password123",
/// };
///
/// let result = login(user).await;
/// ```
///
/// # Arguments
///
/// * `user` - A `User` struct containing the username and password of the user attempting to log in.
///
/// # Returns
///
/// * `Ok(())` - If the user authentication is successful.
/// * `Err(ControlError::ErrorAuthenticate)` - If the password is incorrect or the retrieved data is of an invalid type.
pub async fn login(mut user: User) -> Result<(), ControlError> {
    let start = Instant::now();

    let mut path = match std::env::consts::OS {
        "windows" => var("HOMEPATH").unwrap(),
        _ => var("HOME").unwrap(),
    };

    path.push_str("/.esmeralda/log.log");

    let db = get_database_instance();

    user.password = criptography::encrpt(user.password);

    let data_user = Data::User(user.clone());

    let db_user = db.get(data_user).await.map_err(|err| {

        let _ = log(
            path.clone().into(),
            &format!("[CONTROL] {err:?}\n[CONTROL] Time to login --- {:.3?}", start.elapsed()),
        );

        ControlError::ErrorExternDB(err)
    })?;

    match db_user {
        Data::UserDb(data) => {
            if data.username.is_empty() {
                        
                let _ = log(
                    path.clone().into(),
                    &format!("[CONTROL] Error to find user in system"),
                );

                Err(ControlError::UserNotExists(ErrorLog {
                    title: "User not exists on system",
                    code: 305,
                    file: "control.rs",
                }))
            } else if data.password == user.password {
                gen_user_instance(data);

                let _ = log(
                    path.clone().into(),
                    &format!("[CONTROL] Login successful in {:.3?}", start.elapsed()),
                );
                Ok(())
            } else {
                let _ = log(
                    path.clone().into(),
                    &format!("[CONTROL] Password incorrect --- time of end: {:.3?}", start.elapsed()),
                );

                Err(ControlError::ErrorAuthenticate(ErrorLog {
                    title: "Password incorrect",
                    code: 305,
                    file: "controller.rs",
                }))
            }
        }
        _ => {
            let _ = log(
                path.clone().into(),
                &format!("[CONTROL] Database not accept this format of data --- time of end: {:.3?}", start.elapsed()),
            );

            Err(ControlError::ErrorAuthenticate(ErrorLog {
                title: "Data type received is invalid",
                code: 306,
                file: "controller.rs",
            }))
        }
    }
}

lazy_static! {
    static ref USER_LOGGED: Mutex<Option<UserDb>> = Mutex::new(None);
}

/// Updates the value of the `USER_LOGGED` static variable with the provided `UserDb` instance.
///
/// # Example
///
/// ```
/// let user = UserDb {
///     id: 1,
///     username: String::from("john"),
///     password: String::from("password123"),
/// };
/// gen_user_instance(user);
/// ```
///
/// # Arguments
///
/// * `usr` - A `UserDb` instance representing the user to be stored in the `USER_LOGGED` variable.
pub fn gen_user_instance(usr: UserDb) {
    *USER_LOGGED.lock().unwrap() = Some(UserDb {
        id: usr.id,
        username: usr.username,
        password: usr.password,
    });
}

pub fn get_user_instance() -> std::sync::MutexGuard<'static, Option<UserDb>> {
    USER_LOGGED.lock().unwrap()
}

/// Adds a new user to the database if the provided password matches the user's password.
///
/// # Arguments
///
/// * `new_user` - A struct containing the username, password, and email of the new user.
/// * `password` - The password provided by the user for verification.
///
/// # Returns
///
/// Returns `Ok(())` if the user was successfully added to the database.
/// Returns `Err(ControlError::ErrorToAddUser)` if an error occurred while adding the user, such as empty user data or mismatched passwords.
///
/// # Example
///
/// ```rust
/// # use crate::ControlError;
/// # use crate::ErrorLog;
/// # use crate::Data;
/// # use crate::NewUser;
/// # use crate::get_database_instance;
/// # use crate::encrpt;
/// #
/// # pub async fn add_user(new_user: NewUser, password: String) -> Result<(), ControlError> {
/// #     if new_user.is_empty(){
/// #         Err(ControlError::ErrorToAddUser(ErrorLog {
/// #             title: "No has data for add user",
/// #             code: 305,
/// #             file: "controler.rs"
/// #         }))
/// #     }else if new_user.password == password {
/// #         let db = get_database_instance();
/// #
/// #         let mut new_user = new_user;
/// #         new_user.password = encrpt(new_user.password);
/// #
/// #         let new_user: Data = Data::NewUser(new_user);
/// #
/// #         db.add(new_user).await.map_err(|err| {
/// #             ControlError::ErrorExternDB(err)
/// #         })?;
/// #         Ok(())
/// #     } else {
/// #         Err(ControlError::ErrorToAddUser(ErrorLog {
/// #             title: "Password is not equal",
/// #             code: 305,
/// #             file: "controller.rs",
/// #         }))
/// #     }
/// # }
/// #
/// let new_user = NewUser {
///     username: "john_doe",
///     password: "password123",
///     email: "john@example.com",
/// };
///
/// let password = "password123".to_string();
///
/// let result = add_user(new_user, password).await;
/// ```
pub async fn add_user(new_user: NewUser, password: String) -> Result<(), ControlError> {
    if new_user.is_empty() {
        Err(ControlError::ErrorToAddUser(ErrorLog {
            title: "No has data for add user",
            code: 305,
            file: "controler.rs",
        }))
    } else if new_user.password == password {
        let db = get_database_instance();

        let mut new_user = new_user;
        new_user.password = criptography::encrpt(new_user.password);

        let new_user: Data = Data::NewUser(new_user);

        db.add(new_user)
            .await
            .map_err(|err| ControlError::ErrorExternDB(err))?;
        Ok(())
    } else {
        Err(ControlError::ErrorToAddUser(ErrorLog {
            title: "Password is not equal",
            code: 305,
            file: "controller.rs",
        }))
    }
}

/// Saves the provided data in a file at the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that represents the file path where the data should be saved.
/// * `data` - A reference to the `InterfaceInfo` data that needs to be saved in the file.
///
/// # Returns
///
/// Returns a `Result` containing a string with the saved file path on success, or a `ControlError` on failure.
///
/// # Example
///
/// ```rust
/// # use crate::InterfaceInfo;
/// # use crate::ControlError;
/// # use crate::ErrorLog;
/// # async fn export_csv(path: &str, data: &InterfaceInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// # async fn export_pdf(path: &str, data: &InterfaceInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// # async fn export_html(path: &str, data: &InterfaceInfo) -> Result<String, ControlError> { Ok(String::from("")) }
/// #
/// # #[derive(Debug)]
/// # struct InterfaceInfo;
/// #
/// # #[derive(Debug)]
/// # enum ControlError {
/// #     ErrorValueInvalid(ErrorLog),
/// #     ErrorExtern(ErrorLog),
/// # }
/// #
/// # #[derive(Debug)]
/// # struct ErrorLog {
/// #     title: &'static str,
/// #     code: u32,
/// #     file: &'static str,
/// # }
/// #
/// # async fn save_in_file(path: &str, data: &InterfaceInfo) -> Result<String, ControlError> {
/// let path = "data.csv";
/// let data = InterfaceInfo;
///
/// let result = save_in_file(path, &data).await;
///
/// match result {
///     Ok(path) => println!("File saved successfully at {}", path),
///     Err(error) => println!("Error saving file: {}", error),
/// }
/// # Ok(String::from(""))
/// # }
/// # fn main() {}
/// ```
pub async fn save_in_file(path: &str, data: &InterfaceInfo) -> Result<String, ControlError> {
    let mut extend = path.split('.');

    let extend = extend.nth(1);

    let response = match extend {
        Some("csv") => export_csv(path, data).await,
        Some("pdf") => export_pdf(path, data),
        Some("html") => export_html(path, data).await,
        None => {
            return Err(ControlError::ErrorValueInvalid(ErrorLog {
                title: "Extension not found",
                code: 404,
                file: "controller.rs",
            }))
        }
        _ => {
            return Err(ControlError::ErrorValueInvalid(ErrorLog {
                title: "Extension invalid",
                code: 305,
                file: "controller.rs",
            }))
        }
    };

    match response {
        Ok(path) => Ok(path),
        Err(err) => {

            let mut path = match std::env::consts::OS {
                "windows" => var("HOMEPATH").unwrap(),
                _ => var("HOME").unwrap(),
            };
            
            path.push_str("/.esmeralda/log.log");

            let _ = log(
                path.clone().into(),
                &format!("[CONTROL] {err:?}"),
            );
            Err(ControlError::ErrorExtern(ErrorLog {
                title: "Error in module export",
                code: 500,
                file: "controller.rs",
            }))
        }
    }
}

/// Saves the provided `InterfaceInfo` data to the global database.
///
/// # Arguments
///
/// * `data` - A reference to an `InterfaceInfo` struct containing the data to be saved.
///
/// # Returns
///
/// Returns `Ok(())` if the operation is successful, or an `ErrorExternDB` variant of the `ControlError` enum if there is an error during the database operation.
///
/// # Example
///
/// ```rust
/// # use crate::{InterfaceInfo, ControlError, Data, get_database_instance, get_user_instance};
/// #
/// # pub async fn save(data: &InterfaceInfo) -> Result<(), ControlError>{
///     let db_instance = get_database_instance();
///
///     if let Some(user_logged) = get_user_instance().as_ref(){
///         db_instance.add(Data::Counts(data.clone(), user_logged.clone())).await.map_err(|err| {
///             ControlError::ErrorExternDB(err)
///         })?;
///     }
///
///     Ok(())
/// # }
/// ```
pub async fn save(data: &InterfaceInfo) -> Result<(), ControlError> {
    let db_instance = get_database_instance();

    if let Some(user_logged) = get_user_instance().as_ref() {
        db_instance
            .add(Data::Counts(data.clone(), user_logged.clone()))
            .await
            .map_err(|err| ControlError::ErrorExternDB(err))?;
    }

    Ok(())
}

/// Asynchronously edits the provided `InterfaceInfo` data.
///
/// # Arguments
///
/// * `data` - A reference to an `InterfaceInfo` struct.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure. If successful, `Ok(())` is returned. If an error occurs during the edit operation, an `Err` variant containing a `ControlError` is returned.
///
/// # Example
///
/// ```rust
/// # use crate::ControlError;
/// # use crate::InterfaceInfo;
/// # use crate::Data;
/// #
/// # pub async fn get_database_instance() -> DatabaseInstance { unimplemented!() }
/// # pub fn get_user_instance() -> Option<UserInstance> { unimplemented!() }
/// #
/// # #[derive(Clone)]
/// # pub struct DatabaseInstance;
/// #
/// # impl DatabaseInstance {
/// #     pub async fn edit(&self, data: Data) -> Result<(), ControlError> { unimplemented!() }
/// # }
/// #
/// # #[derive(Clone)]
/// # pub struct UserInstance;
/// #
/// # impl UserInstance {
/// #     pub fn clone(&self) -> UserInstance { unimplemented!() }
/// # }
/// #
/// # #[derive(Clone)]
/// # pub enum Data {
/// #     Counts(InterfaceInfo, UserInstance),
/// # }
/// #
/// # #[derive(Debug)]
/// # pub enum ControlError {
/// #     ErrorExternDB(String),
/// # }
/// #
/// # #[derive(Clone)]
/// # pub struct InterfaceInfo;
/// #
/// # pub async fn edit(data: &InterfaceInfo) -> Result<(), ControlError>{
///     let db_instance = get_database_instance();
///
///     if let Some(user_logged) = get_user_instance().as_ref(){
///         db_instance.edit(Data::Counts(data.clone(), user_logged.clone())).await.map_err(|err| {
///             ControlError::ErrorExternDB(err)
///         })?;
///     }
///
///     Ok(())
/// # }
/// # fn main() {}
/// ```
pub async fn edit(data: &InterfaceInfo) -> Result<(), ControlError> {
    let db_instance = get_database_instance();

    if let Some(user_logged) = get_user_instance().as_ref() {
        db_instance
            .edit(Data::Counts(data.clone(), user_logged.clone()))
            .await
            .map_err(|err| ControlError::ErrorExternDB(err))?;
    }

    Ok(())
}

/// Asynchronously retrieves data from a database and returns it as a result.
///
/// # Example
///
/// ```rust
/// let result = recover().await;
/// ```
///
/// # Errors
///
/// Returns a `ControlError::ErrorExternDB` if an error occurs during the database retrieval.
/// Returns a `ControlError::ErrorExtern` if the recovered data is not of type `Data::Counts`.
///
/// # Returns
///
/// If successful, returns the recovered data as `Ok(data: InterfaceInfo)`.
///
/// # Panics
///
/// This function will panic if the `get_user_instance` function returns `None`.
pub async fn recover() -> Result<InterfaceInfo, ControlError> {
    let data = InterfaceInfo::new();
    let db_instance = get_database_instance();

    let user_logged = get_user_instance().as_ref().unwrap().clone();

    let recovered_data = db_instance
        .get(Data::Counts(data, user_logged.clone()))
        .await
        .map_err(|err| ControlError::ErrorExternDB(err))?;

    match recovered_data {
        Data::Counts(mut data, _) => {
            let id_user_len = user_logged.id.to_string().len();

            let list: Vec<Info> = data
                .list
                .iter_mut()
                .map(|count| {
                    let count_id: String = count.id.to_string();
                    let count_id = count_id.split_at(id_user_len);

                    count.id = count_id.1.trim().parse().unwrap();

                    count.clone()
                })
                .collect();

            data.list = list;

            Ok(data)
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}

pub fn is_alpha(text: &String) -> bool{
    is_alphabetic(text)
}

pub fn is_email(email: &String) -> bool{
    email_valid::validate(email)
}

pub fn signed_month_difference(start: NaiveDate, end: NaiveDate) -> i32 {
    let end_naive = end;
    let start_naive = start;

    let month_diff = end_naive.month() as i32 - start_naive.month() as i32;
    let years_diff = (end_naive.year() - start_naive.year()) as i32;


    if month_diff > 0 {
        (years_diff * 12) + month_diff
    } else {
        (years_diff - 1) * 12 + (month_diff + 12)
    }
}

/// Checks if the given `Info` struct contains all the required information for it to be considered complete.
///
/// # Example
///
/// ```
/// use futures::executor::block_on;
///
/// #[derive(Debug)]
/// struct Info {
///     debtor: String,
///     title: String,
///     value: f64,
///     installments: u32,
/// }
///
/// async fn is_complete(info: &Info) -> bool {
///     if info.debtor.is_empty() || !is_alphabetic(&info.debtor) {
///         return false;
///     } else if info.title.is_empty() {
///         return false;
///     } else if info.value == 0.0 {
///         return false;
///     } else if info.installments == 0 {
///         return false;
///     }
///
///     true
/// }
///
/// fn is_alphabetic(s: &str) -> bool {
///     s.chars().all(|c| c.is_alphabetic() || c == ' ')
/// }
///
/// fn main() {
///     let info = Info {
///         debtor: String::from("John Doe"),
///         title: String::from("Payment"),
///         value: 100.0,
///         installments: 2,
///     };
///
///     let result = block_on(is_complete(&info));
///     println!("Is the info complete? {}", result);
/// }
/// ```
pub async fn is_complete(info: &Info) -> bool {
    if info.debtor.trim().is_empty() || !is_alphabetic(&info.debtor) {
        return false;
    } else if info.title.is_empty() {
        return false;
    } else if info.value == 0.0 {
        return false;
    } else if info.installments == 0 {
        return false;
    }

    true
}