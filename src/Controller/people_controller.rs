use std::{str::FromStr, sync::MutexGuard};

use chrono::NaiveDate;

use super::*;
use crate::prelude::model::People::People;

lazy_static! {
    static ref PEOPLES: Mutex<Vec<People>> = Mutex::new(Vec::new());
}

pub fn gen_peoples_instance(peoples: Vec<People>) {
    *PEOPLES.lock().unwrap() = peoples;
}

pub fn get_peoples_instance() -> std::sync::MutexGuard<'static, Vec<People>> {
    PEOPLES.lock().unwrap()
}

pub async fn get_peoples(
    user_id: &i32,
    db: MutexGuard<'static, DataBase>,
) -> Result<Vec<People>, ControlError> {
    let peoples = Vec::new();
    let data_peoples = Data::People(user_id.clone() as u16, peoples);

    let db_peoples = db.get(data_peoples).await.map_err(|err| {
        tracing::error!("{:?}", err);
        ControlError::ErrorExternDB(err)
    })?;

    match db_peoples {
        Data::People(_, peoples) => Ok(peoples),
        _ => Err(ControlError::ErrorValueInvalid(ErrorLog {
            title: "Data invalid to people",
            code: 500,
            file: "people_controller.rs",
        })),
    }
}

pub async fn add_people(new_people: &People) -> Result<(), ControlError> {
    let db = get_database_instance();

    let user_id = get_user_instance().clone().unwrap().id;
    let data_peoples = Data::People(user_id as u16, vec![new_people.clone()]);

    db.add(data_peoples)
        .await
        .map_err(ControlError::ErrorExternDB)?;

    Ok(())
}

pub async fn edit_people(peoples: Vec<People>) -> Result<(), ControlError> {
    let db = get_database_instance();
    let user_id = get_user_instance().clone().map(|user_db| user_db.get_id());

    if let Some(id) = user_id {
        if !peoples.is_empty() {
            db.edit(Data::People(id as u16, peoples))
                .await
                .map_err(|err| {
                    tracing::error!("{:?}", err);

                    ControlError::ErrorExternDB(err)
                })?;
        }
    }

    Ok(())
}

#[allow(unreachable_code, unused)]
pub async fn delete_people(uid: String) -> Result<(), ControlError> {
    let db = get_database_instance();

    let people = People {
        id: uid,
        address: String::new(),
        name: String::new(),
        wage: 0.0,
        cell_phone: String::new(),
        birthday: NaiveDate::from_str("1970-01-01").unwrap(),
        rg: String::new(),
        cpf: String::new(),
        surname: String::new(),
        voter_registration: String::new(),
        provider: String::new(),
    };
    let user_id = get_user_instance().clone().unwrap().id;
    let data = Data::People(user_id as u16, vec![people]);

    db.delete(data).await.map_err(ControlError::ErrorExternDB)?;
    gen_peoples_instance(get_peoples(&user_id, db).await?);
    Ok(())
}
