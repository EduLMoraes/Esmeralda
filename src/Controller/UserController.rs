use crate::prelude::model::{
    Database::*,
    errors::*,
    User::*,
};
use crate::prelude::segurance::*;
use crate::prelude::Instant;
use crate::prelude::log;
use crate::prelude::env::var;
use std::sync::Mutex;
use lazy_static::lazy_static;


struct UserController;

impl UserController{
    
    lazy_static! {
        static ref USER_LOGGED: Mutex<Option<UserDb>> = Mutex::new(None);
    }
    
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
    
}