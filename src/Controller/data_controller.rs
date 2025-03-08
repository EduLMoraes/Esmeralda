use super::*;
use crate::env;
use crate::model::List::{get_counts_instance, ListCount};
use std::ops::DerefMut;

/// This communicate with the database to save
/// the new data of GLOBAL_COUNTS.
pub async fn save() -> Result<(), ControlError> {
    let db_instance = get_database_instance();
    let data = get_counts_instance().clone();

    if let Some(user_logged) = get_user_instance().as_ref() {
        db_instance
            .add(Data::Counts(data, user_logged.clone(), 0))
            .await
            .map_err(ControlError::ErrorExternDB)?;
    }

    Ok(())
}

pub async fn get_groups() -> Result<Vec<String>, ControlError> {
    let db_instance = get_database_instance();

    if let Some(user_logged) = get_user_instance().as_ref() {
        let data = db_instance
            .get(Data::Groups(Vec::new(), user_logged.id))
            .await
            .map_err(ControlError::ErrorExternDB)?;

        return match data {
            Data::Groups(groups, _) => Ok(groups),
            _ => Err(ControlError::ErrorValueInvalid(ErrorLog {
                title: "Data return is not valid!",
                code: 306,
                file: "data_controller.rs",
            })),
        };
    }

    Err(ControlError::ErrorAuthenticate(ErrorLog {
        title: "No has user instance",
        code: 306,
        file: "data_controller.rs",
    }))
}

/// This communicate with the database to edit
/// the data.
pub async fn edit(data: &ListCount) -> Result<(), ControlError> {
    let db_instance = get_database_instance();

    if let Some(user_logged) = get_user_instance().as_ref() {
        db_instance
            .edit(Data::Counts(data.clone(), user_logged.clone(), 0))
            .await
            .map_err(ControlError::ErrorExternDB)?;
    }

    Ok(())
}

/// This is to recover the years of the payment of database.
pub async fn recover_years() -> Result<Vec<i16>, ControlError> {
    let data = ListCount::new();
    let db_instance = get_database_instance();

    let user_logged = get_user_instance().as_ref().unwrap().clone();

    let recovered_data = db_instance
        .get(Data::Years(data, user_logged.clone()))
        .await
        .map_err(ControlError::ErrorExternDB)?;

    match recovered_data {
        Data::Years(data, _) => {
            get_counts_instance().years = data.years.clone();
            Ok(data.years)
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}

/// This delete a count on databse.
pub async fn delete(id: &i32) -> Result<(), ControlError> {
    let user_id = get_user_instance().as_ref().unwrap().id;

    get_database_instance()
        .delete(Data::Count(id.clone(), user_id))
        .await
        .map_err(|err| ControlError::ErrorExternDB(err))?;

    Ok(())
}
/// This recover all payments and counts of the year existent
/// on database.
///
/// Example of input:
/// ```rs
///     recover(2024);
/// ```
pub async fn recover(year: i16) -> Result<(), ControlError> {
    let data = ListCount::new();
    let db_instance = get_database_instance();

    let user_logged = get_user_instance().as_ref().unwrap().clone();
    env::set_var("YEAR_SELECTED", format!("{}", &year));

    let recovered_data = db_instance
        .get(Data::Counts(data.clone(), user_logged.clone(), year))
        .await
        .map_err(ControlError::ErrorExternDB)?;

    match recovered_data {
        Data::Counts(data, _, _) => {
            get_counts_instance().deref_mut().list = data.list;

            Ok(())
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}

pub async fn update_counts_with_db() -> Result<(), ControlError>{
    let year = env::var("YEAR_SELECTED").map_err(|err| {tracing::error!("YEAR_SELECTED not defined: {:?}", err); ControlError::ErrorExtern(ErrorLog { title: "env YEAR_SELECTED is'n defined", code: 306, file: "control.rs" })})?.parse::<i16>().map_err(|err| {tracing::error!("Failed to parse year: {:?}", err); ControlError::ErrorExtern(ErrorLog { title: "Failed to parse year", code: 306, file: "control.rs" })})?;
    recover(year).await
}