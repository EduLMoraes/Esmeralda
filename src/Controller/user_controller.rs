use super::*;

lazy_static! {
    static ref USER_LOGGED: Mutex<Option<UserDb>> = Mutex::new(None);
}

pub fn gen_user_instance(usr: UserDb) {
    *USER_LOGGED.lock().unwrap() = Some(UserDb {
        id: usr.id,
        username: usr.username,
        password: usr.password,
        email: usr.email,
        last_login: usr.last_login,
    });
}

pub fn get_user_instance() -> std::sync::MutexGuard<'static, Option<UserDb>> {
    USER_LOGGED.lock().unwrap()
}

pub fn exit_user() {
    std::mem::drop(get_user_instance());
    std::mem::drop(get_peoples_instance());
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
            &format!(
                "\n[CONTROL] {err:?}\n[CONTROL] Time to login --- {:.3?}\n",
                start.elapsed()
            ),
        );

        ControlError::ErrorExternDB(err)
    })?;

    match db_user {
        Data::UserDb(data) => {
            if data.username.is_empty() {
                let _ = log(
                    path.clone().into(),
                    "[CONTROL] Error to find user in system\n",
                );

                Err(ControlError::UserNotExists(ErrorLog {
                    title: "User not exists on system",
                    code: 305,
                    file: "control.rs",
                }))
            } else if data.password == user.password {
                db.edit(Data::LastLogin(data.id))
                    .await
                    .map_err(|err| ControlError::ErrorExternDB(err))?;

                let peoples = get_peoples(&data.id, db).await?;
                dbg!(&peoples);
                gen_peoples_instance(peoples);

                gen_user_instance(data);

                let _ = log(
                    path.clone().into(),
                    &format!("[CONTROL] Login successful in {:.3?}\n", start.elapsed()),
                );
                Ok(())
            } else {
                let _ = log(
                    path.clone().into(),
                    &format!(
                        "[CONTROL] Password incorrect --- time of end: {:.3?}\n",
                        start.elapsed()
                    ),
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
                &format!(
                    "[CONTROL] Database not accept this format of data --- time of end: {:.3?}\n",
                    start.elapsed()
                ),
            );

            Err(ControlError::ErrorAuthenticate(ErrorLog {
                title: "Data type received is invalid",
                code: 306,
                file: "controller.rs",
            }))
        }
    }
}

pub async fn add_user(new_user: NewUser) -> Result<(), ControlError> {
    if new_user.is_empty() {
        Err(ControlError::ErrorToAddUser(ErrorLog {
            title: "No has data for add user",
            code: 305,
            file: "controler.rs",
        }))
    } else {
        let db = get_database_instance();

        let mut new_user = new_user;
        new_user.password = criptography::encrpt(new_user.password);

        let new_user: Data = Data::NewUser(new_user);

        db.add(new_user)
            .await
            .map_err(ControlError::ErrorExternDB)?;
        Ok(())
    }
}

pub async fn edit_user(user: UserDb) -> Result<(), ControlError> {
    let db = get_database_instance();

    db.edit(Data::UserDb(user)).await.map_err(|err| {
        dbg!(&err);
        ControlError::ErrorExternDB(err)
    })?;

    Ok(())
}

#[allow(deprecated, unused)]
pub async fn restore_password(user: User) -> Result<(), ControlError> {
    let db = get_database_instance();

    let db_user = db
        .get(Data::User(user))
        .await
        .map_err(ControlError::ErrorExternDB)?;

    match db_user {
        Data::UserDb(mut user_data) => {
            let new_pass = criptography::gen_string(8, &[65, 122]);
            user_data.password = criptography::encrpt(String::from(&new_pass));
            db.edit(Data::UserDb(user_data.clone()))
                .await
                .map_err(ControlError::ErrorExternDB)?;

            std::mem::drop(tokio::task::spawn(async move {
                let _ = send_email(
                    "esmeralda.restorepass@gmail.com",
                    &user_data.email,
                    "Recuperação de senha Esmeralda",
                    format!("Caso não tenha sido você, apenas ignore este e-mail.\n Sua senha agora é: {}", new_pass),
                    String::from("Foi solicitada uma recuperação de senha com seu e-mail")
                    ).await;
            }));
        }
        _ => {
            return Err(ControlError::UserNotExists(ErrorLog {
                title: "User with this email not exists",
                code: 305,
                file: "user_controller.rs",
            }))
        }
    }
    Ok(())
}
