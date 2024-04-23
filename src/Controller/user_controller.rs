use super::*;

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
        name: usr.name,
        username: usr.username,
        password: usr.password,
        email: usr.email,
    });
}

pub fn get_user_instance() -> std::sync::MutexGuard<'static, Option<UserDb>> {
    USER_LOGGED.lock().unwrap()
}

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
                    &format!("[CONTROL] Error to find user in system\n"),
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
            .map_err(|err| ControlError::ErrorExternDB(err))?;
        Ok(())
    }
}

#[allow(deprecated, unused)]
pub async fn restore_password(user: User) -> Result<(), ControlError> {
    let db = get_database_instance();

    let db_user = db
        .get(Data::User(user))
        .await
        .map_err(|err| ControlError::ErrorExternDB(err))?;

    match db_user {
        Data::UserDb(mut user_data) => {
            let new_pass = criptography::gen_string(8, &[65, 122]);
            user_data.password = criptography::encrpt(String::from(&new_pass));
            db.edit(Data::UserDb(user_data.clone()))
                .await
                .map_err(|err| ControlError::ErrorExternDB(err))?;

            let _ = tokio::task::spawn(async move {
                let _ = send_email(
                    "esmeralda.restorepass@gmail.com",
                    &user_data.email,
                    "Recuperação de senha Esmeralda",
                    String::from(format!("Caso não tenha sido você, apenas ignore este e-mail.\n Sua senha agora é: {}", new_pass)),
                    String::from("Foi solicitada uma recuperação de senha com seu e-mail")
                    ).await;
            });
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
