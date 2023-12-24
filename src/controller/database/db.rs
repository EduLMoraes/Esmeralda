#[path = "../config/to_db.rs"]
mod to_db;
mod error_db;

use crate::errors::ErrorLog;
use crate::structs::*; 
use crate::structs_db::*; 
use crate::Error;
use deadpool_postgres::{Pool, Runtime, GenericClient};
use postgres::{NoTls, Statement};
use lazy_static::lazy_static;
use std::sync::Mutex;



#[allow(dead_code)]
pub struct DataBase {
    pub pool: Pool,
}

#[allow(dead_code)]
impl DataBase {
    pub fn new() -> Result<Self, DataBaseError> {
        let db = DataBase {
            pool: to_db::get_config()
                .map_err(|_| {
                    DataBaseError::GetConfigError(ErrorLog {
                        title: "Config no error",
                        code: 802,
                        file: "db.rs",
                    })
                })?
                .create_pool(Some(Runtime::Tokio1), NoTls)
                .map_err(|_| {
                    DataBaseError::CreatePoolError(ErrorLog {
                        title: "Pool not found",
                        code: 802,
                        file: "db.rs",
                    })
                })?,
            };
            
        Ok(db)
    }

    pub async fn add(&self, data: Data) -> Result<(), DataBaseError> {
        match data {
            Data::NewUser(user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                let stmt: Statement = conn.prepare("INSERT INTO users (username, password, email) VALUES ($1, $2, $3) ").await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to prepare query",
                        code: 808,
                        file: "db.rs",
                    })
                })?;

                conn.execute(&stmt, &[&user.username, &user.password, &user.email]).await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to execute query",
                        code: 808,
                        file: "db.rs",
                    })
                })?;

                Ok(())
            },
            Data::Counts(counts, user) => {
                let conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                for i in 0..counts.len(){
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
                            status
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ").await.map_err(|_| {
                        DataBaseError::AddUserError(ErrorLog {
                            title: "Error to prepare query",
                            code: 808,
                            file: "db.rs",
                        })
                    })?;
    
                    conn.execute(&stmt, &[
                            &counts.list[i].id,
                            &user.id, 
                            &counts.list[i].debtor, 
                            &counts.list[i].title, 
                            &counts.list[i].description, 
                            &counts.list[i].value, 
                            &counts.list[i].paid_installments, 
                            &counts.list[i].installments, 
                            &counts.list[i].date_in.to_string(), 
                            &counts.list[i].date_out.to_string(), 
                            &counts.list[i].status
                        ])
                        .await.map_err(|_| {
                            DataBaseError::AddUserError(ErrorLog {
                                title: "Error to execute query",
                                code: 808,
                                file: "db.rs",
                            })
                        })?;
                }

                Ok(())
            },
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "db.rs",
            })),
        }
    }

    pub async fn get(&self, data: Data) -> Result<Data, DataBaseError> {
        match data {
            Data::User(user) => {
                let conn = self.pool.get().await.map_err(|e| {
                    println!("{:?}", e);
                    DataBaseError::GetUserError(ErrorLog {
                        title: "Error to get Object<Manager>",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                let stmt = conn.prepare("SELECT * FROM users WHERE username = $1 ").await.map_err(|_| {
                    DataBaseError::GetUserError(ErrorLog {
                        title: "Error to prepare query to get user",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                let row = conn.query_one(&stmt, &[&user.username]).await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "User not found!",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                let user = UserDb {
                    id: row.get("id"),
                    username: row.get("username"),
                    password: row.get("password"),
                };

                Ok(Data::UserDb(user))
            },
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "db.rs",
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

#[allow(dead_code)]
pub enum Data {
    NewUser(NewUser),
    User(User),
    UserDb(UserDb),
    Counts(InterfaceInfo, UserDb),
}

#[derive(Error, Debug, PartialEq)]
pub enum DataBaseError {
    #[error("Config error")]
    GetConfigError(ErrorLog<'static>),

    #[error("Require pool error")]
    CreatePoolError(ErrorLog<'static>),

    #[error("Add user not working")]
    AddUserError(ErrorLog<'static>),

    #[error("Config error")]
    GetUserError(ErrorLog<'static>),

    #[error("DataType not Acept")]
    DataTypeInvalid(ErrorLog<'static>),
}
