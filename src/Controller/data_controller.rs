use super::*;
use crate::model::list::InterfaceInfo;
use crate::std::sync::Mutex;
use crate::std::sync::OnceLock;

pub static mut GLOBAL_COUNTS: OnceLock<InterfaceInfo> = OnceLock::new();

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

            unsafe {
                let teste = GLOBAL_COUNTS.get_mut();
                match teste {
                    Some(_) => {}
                    None => GLOBAL_COUNTS = OnceLock::from(data.clone()),
                }
            }

            Ok(data)
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}
