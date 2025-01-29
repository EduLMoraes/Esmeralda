use std::sync::MutexGuard;

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
        dbg!(&err);
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

    let data_peoples = Data::People(
        get_user_instance().clone().unwrap().id as u16,
        vec![new_people.clone()],
    );

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
                    dbg!(&err);
                    ControlError::ErrorExternDB(err)
                })?;
        }
    }

    Ok(())
}
