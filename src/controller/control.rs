use super::cryptography::encrpt;
use super::errors::*;
use super::structs::*;
use super::db::*;
use crate::export::*;
use crate::structs_db::*;
use crate::alphabetic::is_alphabetic;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub async fn login(mut user: User) -> Result<(), ControlError>{
    let db = get_database_instance();

    user.password = encrpt(user.password);

    let data_user = Data::User(user.clone());

    let db_user = db.get(data_user).await.map_err(|err| {
        ControlError::ErrorExternDB(err)
    })?;

    match db_user {
        Data::UserDb(data) => {
            if data.password == user.password{
                gen_user_instance(data);

                Ok(())
            }else {
                Err(ControlError::ErrorAuthenticate(
                    ErrorLog { title: "Password incorrect", code: 305, file: "controller.rs" }
                ))
            }
        },
        _ => Err(ControlError::ErrorAuthenticate(
            ErrorLog { title: "Data type received is invalid", code: 306, file: "controller.rs" }
        ))
    }
}
 
lazy_static! {
    static ref USER_LOGGED: Mutex<Option<UserDb>> = Mutex::new(None);
}

fn gen_user_instance(usr: UserDb){
    *USER_LOGGED.lock().unwrap() = Some(UserDb {
        id: usr.id,
        username: usr.username,
        password: usr.password,
     });
}

fn get_user_instance() -> std::sync::MutexGuard<'static, Option<UserDb>> {
    USER_LOGGED.lock().unwrap()
}

pub async fn add_user(new_user: NewUser, password: String) -> Result<(), ControlError> {
    if new_user.is_empty(){
        Err(ControlError::ErrorToAddUser(ErrorLog { 
            title: "No has data for add user", 
            code: 305, 
            file: "controler.rs" 
        }))
    }else if new_user.password == password {
        let db = get_database_instance();

        let mut new_user = new_user;
        new_user.password = encrpt(new_user.password);

        let new_user: Data = Data::NewUser(new_user);

        db.add(new_user).await.map_err(|err| {
            ControlError::ErrorExternDB(err)
        })?;
        Ok(())
    } else {
        Err(ControlError::ErrorToAddUser(ErrorLog {
            title: "Password is not equal",
            code: 305,
            file: "controller.rs",
        }))
    }
}

pub async fn save_in_file(path: &str, data: &InterfaceInfo) -> Result<String, ControlError>{
    let mut extend = path.split('.');

    let extend = extend.nth(1);

    let response = match extend {
        Some("csv") => export_csv(path, data).await,
        Some("pdf")=> export_pdf(path, data),
        Some("html") => export_html(path, data).await,
        None => return Err(ControlError::ErrorValueInvalid(
            ErrorLog { title: "Extension not found", code: 404, file: "controller.rs" }
        )),
        _ => return Err(ControlError::ErrorValueInvalid(
            ErrorLog { title: "Extension invalid", code: 305, file: "controller.rs" }
        ))
    };

    match response{
        Ok(path) => Ok(path),
        Err(e) => {
            println!("{}", e);
            Err(ControlError::ErrorExtern(
                ErrorLog{ title: "Error in module export", code: 500, file: "controller.rs"}
            ))
        },
    }

}

pub async fn save(data: &InterfaceInfo) -> Result<(), ControlError>{
    let db_instance = get_database_instance();
    
    if let Some(user_logged) = get_user_instance().as_ref(){
        db_instance.add(Data::Counts(data.clone(), user_logged.clone())).await.map_err(|err| {
            ControlError::ErrorExternDB(err)
        })?;
    }

    Ok(())
}

pub async fn edit(data: &InterfaceInfo) -> Result<(), ControlError>{
    let db_instance = get_database_instance();
    
    if let Some(user_logged) = get_user_instance().as_ref(){
        db_instance.edit(Data::Counts(data.clone(), user_logged.clone())).await.map_err(|err| {
            ControlError::ErrorExternDB(err)
        })?;
    }

    Ok(())
}

pub async fn recover() -> Result<InterfaceInfo, ControlError>{
    let data = InterfaceInfo::new();
    let db_instance = get_database_instance();
    
    let user_logged = get_user_instance().as_ref().unwrap().clone();

    let recovered_data = db_instance.get(Data::Counts(data, user_logged)).await.map_err(|err| {
            ControlError::ErrorExternDB(err)
    })?;

    match recovered_data{
        Data::Counts(data, _) => Ok(data),
        _ => Err(ControlError::ErrorExtern(ErrorLog { title: "Error to recover data", code: 306, file: "control.rs" })),
    }
}

pub async fn is_complete(info: &Info) -> bool{
    if info.debtor.is_empty() || !is_alphabetic(&info.debtor){
        return false;
    }else if info.title.is_empty(){
        return false;
    }else if info.value == 0.0{
        return false;
    }else if info.installments == 0{
        return false;
    }
    
    true
}



