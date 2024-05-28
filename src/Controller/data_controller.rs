use super::*;
use crate::model::List::ListCount;
use std::borrow::{Borrow, BorrowMut};

pub static mut GLOBAL_COUNTS: ListCount = ListCount {
    list: Vec::new(),
    years: Vec::new(),
};

/// This communicate with the database to save
/// the new data of GLOBAL_COUNTS.
pub async fn save() -> Result<(), ControlError> {
    let db_instance = get_database_instance();
    let data = unsafe { GLOBAL_COUNTS.borrow() };

    if let Some(user_logged) = get_user_instance().as_ref() {
        db_instance
            .add(Data::Counts(data.clone(), user_logged.clone(), 0))
            .await
            .map_err(|err| ControlError::ErrorExternDB(err))?;
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
            .map_err(|err| ControlError::ErrorExternDB(err))?;
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
        .map_err(|err| ControlError::ErrorExternDB(err))?;

    match recovered_data {
        Data::Years(data, _) => {
            unsafe {
                let global = GLOBAL_COUNTS.borrow_mut();
                global.years = data.years.clone()
            }

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

    let recovered_data = db_instance
        .get(Data::Counts(data.clone(), user_logged.clone(), year))
        .await
        .map_err(|err| ControlError::ErrorExternDB(err))?;

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

            unsafe {
                GLOBAL_COUNTS.list = data.list;
            }

            use crate::utils::export::svg;
            unsafe {
                svg::to_svg(
                    2024,
                    GLOBAL_COUNTS.get_data_months(),
                    GLOBAL_COUNTS.filter_debtors(),
                );
            }
            Ok(())
        }
        _ => Err(ControlError::ErrorExtern(ErrorLog {
            title: "Error to recover data",
            code: 306,
            file: "control.rs",
        })),
    }
}
