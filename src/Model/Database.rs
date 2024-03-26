use crate::prelude::control::config::database::get_config;
use crate::prelude::env::var;
use crate::prelude::log;
use crate::prelude::model::{
    errors::{DataBaseError, ErrorLog},
    Count::Count,
    List::ListCount,
    User::*,
};
use chrono::NaiveDate;
use deadpool_postgres::{GenericClient, Pool, Runtime};
use lazy_static::lazy_static;
use postgres::{NoTls, Statement};
use std::sync::Mutex;

/// This code snippet defines a `DataBase` struct in Rust. It contains methods for creating a new database connection, adding data to the database, getting data from the database, and editing data in the database.

/// Example Usage:
/// ```
/// let db = DataBase::new()?;
/// db.add(Data::NewUser(user))?;
/// let data = db.get(Data::User(user))?;
/// db.edit(Data::Counts(counts, user))?;
/// ```

/// Inputs:
/// - `data`: An enum that represents different types of data to be added, retrieved, or edited in the database.
/// - `user`: An instance of the `User` struct that contains user information.
/// - `counts`: An instance of the `ListCount` struct that contains a list of `Count` structs.

/// Outputs:
/// - `Result<Self, DataBaseError>`: The `new` method returns a `Result` with either a `DataBase` instance or a `DataBaseError`.
/// - `Result<(), DataBaseError>`: The `add` and `edit` methods return a `Result` with either an empty value or a `DataBaseError`.
/// - `Result<Data, DataBaseError>`: The `get` method returns a `Result` with either a `Data` enum or a `DataBaseError`.
///
#[allow(dead_code)]
pub struct DataBase {
    pub pool: Pool,
}

#[allow(dead_code)]
impl DataBase {
    pub fn new() -> Result<Self, DataBaseError> {
        let db = DataBase {
            pool: get_config()
                .map_err(|_| {
                    DataBaseError::GetConfigError(ErrorLog {
                        title: "Config no error",
                        code: 802,
                        file: "Database.rs",
                    })
                })?
                .create_pool(Some(Runtime::Tokio1), NoTls)
                .map_err(|_| {
                    DataBaseError::CreatePoolError(ErrorLog {
                        title: "Pool not found",
                        code: 802,
                        file: "Database.rs",
                    })
                })?,
        };

        Ok(db)
    }

    pub async fn add(&self, data: Data) -> Result<(), DataBaseError> {
        let mut path = match std::env::consts::OS {
            "windows" => var("HOMEPATH").unwrap(),
            _ => var("HOME").unwrap(),
        };

        path.push_str("/.esmeralda/log.log");

        match data {
            Data::NewUser(user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let stmt: Statement = conn
                    .prepare("INSERT INTO users (username, password, email) VALUES ($1, $2, $3) ")
                    .await
                    .map_err(|_| {
                        DataBaseError::AddUserError(ErrorLog {
                            title: "Error to prepare query",
                            code: 808,
                            file: "Database.rs",
                        })
                    })?;

                conn.execute(&stmt, &[&user.username, &user.password, &user.email])
                    .await
                    .map_err(|err| {
                        let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                        DataBaseError::AddUserError(ErrorLog {
                            title: "Error to execute query",
                            code: 808,
                            file: "Database.rs",
                        })
                    })?;

                Ok(())
            }
            Data::Counts(mut counts, user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let stmt: Statement = conn.prepare("INSERT INTO counts (
                        count_id,
                        user_id, 
                        debtor, 
                        title, 
                        description, 
                        value, 
                        paid_installments, 
                        installments, 
                        date_in, 
                        date_out, 
                        status,
                        nature
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, TO_DATE($9, 'YYYY-MM-DD'), TO_DATE($10, 'YYYY-MM-DD'), $11, $12) ").await.map_err(|_| {
                    DataBaseError::AddCountError(ErrorLog {
                        title: "Error to prepare query",
                        code: 808,
                        file: "Database.rs",
                    })
                })?;

                let _ = conn
                    .execute(
                        &stmt,
                        &[
                            &format!("{}{}", user.id, counts.list[0].id)
                                .trim()
                                .parse::<i32>()
                                .unwrap(),
                            &user.id,
                            &counts.list[0].debtor,
                            &counts.list[0].title,
                            &counts.list[0].description,
                            &counts.list[0].value,
                            &(counts.list[0].paid_installments as i32),
                            &(counts.list[0].installments as i32),
                            &counts.list[0].date_in.to_string(),
                            &counts.list[0].date_out.to_string(),
                            &counts.list[0].status,
                            &counts.list[0].nature,
                        ],
                    )
                    .await
                    .map_err(|err| {
                        let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                        DataBaseError::AddUserError(ErrorLog {
                            title: "Error to execute query",
                            code: 808,
                            file: "Database.rs",
                        })
                    });

                Ok(())
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })),
        }
    }

    pub async fn get(&self, data: Data) -> Result<Data, DataBaseError> {
        let mut path = match std::env::consts::OS {
            "windows" => var("HOMEPATH").unwrap(),
            _ => var("HOME").unwrap(),
        };

        path.push_str("/.esmeralda/log.log");

        match data {
            Data::User(user) => {
                let conn = self.pool.get().await.map_err(|err| {
                    let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                    DataBaseError::GetUserError(ErrorLog {
                        title: "Error to get Object<Manager>",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let stmt = conn
                    .prepare("SELECT * FROM users WHERE username = $1 LIMIT 1")
                    .await
                    .map_err(|_| {
                        DataBaseError::GetUserError(ErrorLog {
                            title: "Error to prepare query to get user",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                let row = conn
                    .query_one(&stmt, &[&user.username])
                    .await
                    .map_err(|_| {
                        DataBaseError::GetUserError(ErrorLog {
                            title: "User not found!",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                let id = row.get::<_, i32>("user_id");

                let user = UserDb {
                    id: id,
                    username: row.get("username"),
                    password: row.get("password"),
                    email: row.get("email"),
                };

                Ok(Data::UserDb(user))
            }
            Data::Counts(mut l_counts, user) => {
                let conn = self.pool.get().await.map_err(|err| {
                    let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                    DataBaseError::GetUserError(ErrorLog {
                        title: "Error to get Object<Manager>",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let stmt = conn
                    .prepare(
                        "SELECT 
                        TO_CHAR(date_in, 'YYYY-MM-DD') AS date_in, 
                        TO_CHAR(date_out, 'YYYY-MM-DD') AS date_out, 
                        * FROM counts WHERE user_id = $1
                        ORDER BY count_id",
                    )
                    .await
                    .map_err(|_| {
                        DataBaseError::GetCountsError(ErrorLog {
                            title: "Error to prepare query to get user",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                let rows = conn.query(&stmt, &[&user.id]).await.map_err(|_| {
                    DataBaseError::GetCountsError(ErrorLog {
                        title: "User not found!",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let mut counts: Vec<Count> = Vec::new();
                for row in rows {
                    let count = Count {
                        id: row.get::<_, i32>("count_id"),
                        debtor: row.get::<_, String>("debtor"),
                        title: row.get::<_, String>("title"),
                        description: row.get::<_, String>("description"),
                        value: row.get::<_, f32>("value"),
                        date_in: row
                            .get::<_, String>("date_in")
                            .parse::<NaiveDate>()
                            .unwrap(),
                        date_out: row
                            .get::<_, String>("date_out")
                            .parse::<NaiveDate>()
                            .unwrap(),
                        paid_installments: row
                            .get::<_, i32>("paid_installments")
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        installments: row
                            .get::<_, i32>("installments")
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        status: row.get::<_, bool>("status"),
                        nature: row.get::<_, String>("nature"),
                    };

                    counts.insert(0, count);
                }

                l_counts.list = counts;
                Ok(Data::Counts(l_counts, user))
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })),
        }
    }

    pub async fn edit(&self, data: Data) -> Result<(), DataBaseError> {
        let mut path = match std::env::consts::OS {
            "windows" => var("HOMEPATH").unwrap(),
            _ => var("HOME").unwrap(),
        };

        path.push_str("/.esmeralda/log.log");

        match data {
            Data::Counts(counts, user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::EditCountsError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                for i in 0..counts.len() {
                    let stmt: Statement = conn
                        .prepare(
                            "UPDATE counts SET
                            debtor = $3, 
                            title = $4, 
                            description = $5, 
                            value = $6,  
                            paid_installments = $7, 
                            installments = $8, 
                            date_in = TO_DATE($9, 'YYYY-MM-DD'), 
                            date_out = TO_DATE($10, 'YYYY-MM-DD'), 
                            status = $11, 
                            nature = $12
                            WHERE count_id = $1 AND user_id = $2",
                        )
                        .await
                        .map_err(|_| {
                            DataBaseError::EditCountsError(ErrorLog {
                                title: "Error to prepare query",
                                code: 808,
                                file: "Database.rs",
                            })
                        })?;

                    conn.execute(
                        &stmt,
                        &[
                            &format!("{}{}", user.id, counts.list[i].id)
                                .trim()
                                .parse::<i32>()
                                .unwrap(),
                            &user.id,
                            &counts.list[i].debtor,
                            &counts.list[i].title,
                            &counts.list[i].description,
                            &counts.list[i].value,
                            &(counts.list[i].paid_installments as i32),
                            &(counts.list[i].installments as i32),
                            &counts.list[i].date_in.to_string(),
                            &counts.list[i].date_out.to_string(),
                            &counts.list[i].status,
                            &counts.list[i].nature,
                        ],
                    )
                    .await
                    .map_err(|err| {
                        let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                        DataBaseError::AddUserError(ErrorLog {
                            title: "Error to execute query",
                            code: 808,
                            file: "Database.rs",
                        })
                    })?;
                }

                Ok(())
            }
            Data::UserDb(user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let query: Statement = conn
                    .prepare(
                        "UPDATE users SET
                            password = $1
                            WHERE user_id = $2",
                    )
                    .await
                    .map_err(|_| {
                        DataBaseError::EditUserError(ErrorLog {
                            title: "Error to prepare query",
                            code: 808,
                            file: "Database.rs",
                        })
                    })?;

                conn.execute(&query, &[&user.password, &user.id])
                    .await
                    .map_err(|err| {
                        let _ = log(path.clone().into(), &format!("[DATABASE] {err:?}\n"));

                        DataBaseError::EditUserError(ErrorLog {
                            title: "Error to execute query",
                            code: 808,
                            file: "Database.rs",
                        })
                    })?;

                Ok(())
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })),
        }
    }
}

lazy_static! {
    static ref GLOBAL_DATABASE: Mutex<DataBase> = Mutex::new(DataBase::new().unwrap());
}

pub fn get_database_instance() -> std::sync::MutexGuard<'static, DataBase> {
    GLOBAL_DATABASE.lock().unwrap()
}

/// Represents different types of data that can be used as input for a function or method.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Data {
    NewUser(NewUser),
    User(User),
    UserDb(UserDb),
    Counts(ListCount, UserDb),
}
