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
        Data::Counts(mut data, _, _) => {
            let id_user_len = user_logged.id.to_string().len();

            let list: Vec<Count> = data
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

            get_counts_instance().deref_mut().list = data.list;

            // binding.list = data.list;

            Ok(())
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}
