use crate::{errors::ErrorLog, structs::*, var, Error};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use postgres::NoTls;
use lazy_static::lazy_static;
use std::sync::Mutex;

/// Configuração do banco de dados.
#[allow(dead_code)]
pub fn get_db_config() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();

    config.user = Some(var("DB_USER").unwrap_or_else(|_| "postgres".into()));
    config.password = Some(var("DB_PASSWORD").unwrap_or_else(|_| "postgres".into()));
    config.dbname = Some(var("DB_NAME").unwrap_or_else(|_| "postgres".into()));
    config.host = Some(var("DB_HOSTNAME").unwrap_or_else(|_| "localhost".into()));
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    Ok(config)
}

#[allow(dead_code)]
pub struct DataBase {
    pub pool: Pool,
}

#[allow(dead_code)]
impl DataBase {
    pub fn new() -> Result<Self, DataBaseError> {
        let db = DataBase {
            pool: get_db_config()
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
            
        println!("Database conectado!");
        Ok(db)
    }

    pub async fn add(&self, data: Data) -> Result<(), DataBaseError> {
        match data {
            Data::NewUser(_user) => {
                let _conn = self.pool.get().await.map_err(|_| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to get pool",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                Ok(())
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "db.rs",
            })),
        }
    }

    pub async fn get(&self, data: Data) -> Result<(), DataBaseError> {
        match data {
            Data::User(_user) => {
                let _conn = self.pool.get().await.map_err(|e| {
                    println!("{:?}", e);
                    DataBaseError::GetUserError(ErrorLog {
                        title: "Error to get Object<Manager>",
                        code: 804,
                        file: "db.rs",
                    })
                })?;

                Ok(())
            }
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
    User(User)
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
