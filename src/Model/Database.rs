use crate::prelude::env::var;
// use crate::prelude::log;
use crate::prelude::model::{
    errors::{DataBaseError, ErrorLog},
    Count::Count,
    List::ListCount,
    User::*,
};
use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use std::sync::Mutex;

#[allow(dead_code)]
pub struct DataBase {
    pub pool: Connection,
}

#[allow(dead_code)]
impl DataBase {
    pub fn new() -> Result<Self, DataBaseError> {
        let db = DataBase {
            pool: match Connection::open(var("DB_PATH").unwrap())
                .map_err(|_| {
                    DataBaseError::CreatePoolError(ErrorLog {
                        title: "Error to connect database",
                        code: 812,
                        file: "Database.rs",
                    })
                })?
                .execute_batch("SELECT * FROM users NATURAL JOIN counts")
            {
                Ok(_) => Connection::open(var("DB_PATH").unwrap()).map_err(|_| {
                    DataBaseError::CreatePoolError(ErrorLog {
                        title: "Error to connect database",
                        code: 812,
                        file: "Database.rs",
                    })
                })?,
                Err(_) => {
                    let conn = Connection::open(var("DB_PATH").unwrap()).map_err(|_| {
                        DataBaseError::CreatePoolError(ErrorLog {
                            title: "Error to connect database",
                            code: 812,
                            file: "Database.rs",
                        })
                    })?;
                    conn.execute_batch(
                        "
                        CREATE TABLE IF NOT EXISTS users (
                            user_id INTEGER PRIMARY KEY,
                            username VARCHAR(50) NOT NULL UNIQUE,
                            email VARCHAR(100) NOT NULL UNIQUE,
                            password VARCHAR(200) NOT NULL,
                            name VARCHAR(100) NOT NULL,
                            wage REAL NOT NULL,
                            last_login DATE
                        );
                        
                        CREATE TABLE IF NOT EXISTS counts (
                            count_id INTEGER PRIMARY KEY,
                            user_id INTEGER NOT NULL,
                            debtor VARCHAR(100) NOT NULL,
                            title VARCHAR(50) NOT NULL,
                            description TEXT,
                            value REAL NOT NULL,
                            paid_installments INTEGER,
                            installments INTEGER DEFAULT 1,
                            date_in DATE NOT NULL,
                            date_out DATE NOT NULL,
                            status BOOLEAN NOT NULL,
                            nature VARCHAR(15) NOT NULL,
                            FOREIGN KEY (user_id) REFERENCES users
                        ); 
                    ",
                    )
                    .map_err(|_| {
                        DataBaseError::CreatePoolError(ErrorLog {
                            title: "Create tables failed!",
                            code: 500,
                            file: "Database.rs",
                        })
                    })?;

                    conn
                }
            },
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
                let mut stmt = self.pool
                    .prepare("INSERT INTO users (username, password, email, name, wage) VALUES (?1, ?2, ?3, ?4, ?5) ")
                    .unwrap();

                stmt.execute([
                    user.username,
                    user.password,
                    user.email,
                    user.name,
                    user.wage.to_string(),
                ])
                .map_err(|_err| {
                    DataBaseError::AddUserError(ErrorLog {
                        title: "User already existis",
                        code: 500,
                        file: "Database.rs",
                    })
                })?;

                Ok(())
            }
            Data::Counts(counts, user, _year) => {
                self.pool
                    .prepare(
                        "INSERT INTO counts (
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
                    ) VALUES (
                        (
                            select(
                                    select user_id from users where user_id = ?1
                                ) || coalesce((
                                    max(
                                        substr(count_id, 2)
                                    +1)
                                ), 1)
                            from counts 
                            where user_id = ?1
                        ), 
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
                    strftime('%Y-%m-%d', ?8), strftime('%Y-%m-%d', ?9), 
                    ?10, ?11) ",
                    )
                    .map_err(|er| {
                        println!("{er}");
                        DataBaseError::AddCountError(ErrorLog {
                            title: "Error to prepare statement",
                            code: 301,
                            file: "database.rs",
                        })
                    })?
                    .execute(params![
                        user.id,
                        counts.list[0].debtor,
                        counts.list[0].title,
                        counts.list[0].description,
                        counts.list[0].value,
                        counts.list[0].paid_installments,
                        counts.list[0].installments,
                        counts.list[0].date_in.to_string(),
                        counts.list[0].date_out.to_string(),
                        counts.list[0].status,
                        counts.list[0].nature
                    ])
                    .map_err(|er| {
                        println!("{er}");
                        DataBaseError::AddCountError(ErrorLog {
                            title: "Error to prepare statement",
                            code: 301,
                            file: "database.rs",
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

    pub async fn get(&self, data: Data) -> Result<Data, DataBaseError> {
        let mut path = match std::env::consts::OS {
            "windows" => var("HOMEPATH").unwrap(),
            _ => var("HOME").unwrap(),
        };

        path.push_str("/.esmeralda/log.log");

        match data {
            Data::User(user) => {
                let mut stmt = self
                    .pool
                    .prepare("SELECT user_id, name, username, password, email, coalesce(strftime('%m', last_login), '0') as last_login FROM users WHERE username = ?1 LIMIT 1")
                    .map_err( |_| {
                        match self.pool.execute_batch("
                            ALTER TABLE users ADD COLUMN last_login DATE;
                        ").map_err(|_|
                            { DataBaseError::GetUserError(ErrorLog { title: "Not added columns", code: 804, file: "Database.rs" })}
                        ){
                            Ok(_) => {},
                            Err(err) => {return err}
                        }

                        DataBaseError::GetUserError(ErrorLog {
                            title: "stmt not working!",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                let user = stmt
                    .query_row([user.username], |row| {
                        Ok(UserDb {
                            id: row
                                .get::<_, i32>("user_id")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get id value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                            name: row
                                .get::<_, String>("name")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get name value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                            username: row
                                .get::<_, String>("username")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get username value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                            password: row
                                .get::<_, String>("password")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get password value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                            email: row
                                .get::<_, String>("email")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get email value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                            last_login: row
                                .get::<_, String>("last_login")
                                .map_err(|_| {
                                    DataBaseError::GetUserError(ErrorLog {
                                        title: "Error to get last_login value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap(),
                        })
                    })
                    .map_err(|_| {
                        DataBaseError::GetUserError(ErrorLog {
                            title: "Error to get user in database",
                            code: 500,
                            file: "Database.rs",
                        })
                    })?;

                Ok(Data::UserDb(user))
            }
            Data::Counts(mut l_counts, user, year) => {
                let mut stmt = self.pool
                    .prepare(
                        "SELECT count_id, user_id,
                        strftime('%Y-%m-%d', date_in) AS date_in, strftime('%Y-%m-%d', date_out) AS date_out, 
                        debtor, title, description, value, paid_installments, installments, status, nature
                        FROM counts
                        WHERE user_id = ?1
                        AND
                        ( strftime('%Y', date_in) like ?2
                        OR strftime('%Y', date_out) like ?2 )
                        order by count_id",
                    )
                    .map_err(|_| {
                        DataBaseError::GetCountsError(ErrorLog {
                            title: "Error to prepare query to get user",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                let mut rows = stmt.query([&user.id, &(year as i32)]).map_err(|e| {
                    println!("{e}");
                    DataBaseError::GetCountsError(ErrorLog {
                        title: "Query to counts not working!",
                        code: 804,
                        file: "Database.rs",
                    })
                })?;

                let mut counts: Vec<Count> = Vec::new();
                let natures = [
                    "Casa",
                    "Transporte",
                    "Saúde",
                    "Lazer",
                    "Alimentação",
                    "Receita",
                ];

                while let Ok(Some(row)) = rows.next() {
                    let count = Count {
                        id: row
                            .get::<_, i32>("count_id")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get id value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap(),
                        debtor: row
                            .get::<_, String>("debtor")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get debtor value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap(),
                        title: row
                            .get::<_, String>("title")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get title value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap(),
                        description: row
                            .get::<_, String>("description")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get description value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap(),
                        value: row
                            .get::<_, f32>("value")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap(),
                        date_in: row
                            .get::<_, String>("date_in")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get date_in value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap()
                            .parse::<NaiveDate>()
                            .unwrap(),
                        date_out: row
                            .get::<_, String>("date_out")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get date_out value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap()
                            .parse::<NaiveDate>()
                            .unwrap(),
                        paid_installments: row
                            .get::<_, i32>("paid_installments")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get paid_installments value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        installments: row
                            .get::<_, i32>("installments")
                            .map_err(|_| {
                                DataBaseError::GetCountsError(ErrorLog {
                                    title: "Error to get installments value",
                                    code: 500,
                                    file: "Database.rs",
                                })
                            })
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        status: row.get::<_, i64>("status").map_err(|err| {
                            println!("{:?}", err);
                            DataBaseError::GetCountsError(ErrorLog {
                                title: "Error to get status value",
                                code: 500,
                                file: "Database.rs",
                            })
                        })? > 0,
                        nature: if natures.contains(
                            &row.get::<_, String>("nature")
                                .map_err(|_| {
                                    DataBaseError::GetCountsError(ErrorLog {
                                        title: "Error to get nature value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap()
                                .trim(),
                        ) {
                            row.get::<_, String>("nature")
                                .map_err(|_| {
                                    DataBaseError::GetCountsError(ErrorLog {
                                        title: "Error to get nature value",
                                        code: 500,
                                        file: "Database.rs",
                                    })
                                })
                                .unwrap()
                        } else {
                            String::from("Outros")
                        },
                    };

                    counts.insert(0, count);
                }

                l_counts.list = counts;
                Ok(Data::Counts(l_counts, user, year))
            }
            Data::Years(mut l_counts, user) => {
                let mut stmt = self
                    .pool
                    .prepare(
                        "SELECT 
                        DISTINCT strftime('%Y', date_out) as date_out
                        FROM counts 
                        WHERE
                        user_id = ?1",
                    )
                    .unwrap();

                let mut rows = stmt.query([user.id as i64]).unwrap();

                let mut years: Vec<i16> = Vec::new();

                while let Ok(Some(row)) = rows.next() {
                    years.push(
                        row.get::<_, String>("date_out")
                            .unwrap()
                            .parse::<i16>()
                            .unwrap(),
                    );
                }

                if years.is_empty() {
                    years.push(chrono::Utc::now().year() as i16);
                }

                l_counts.years = years;

                Ok(Data::Years(l_counts, user))
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
            Data::Counts(counts, user, _) => {
                for i in 0..counts.len() {
                    self.pool
                        .prepare(
                            "UPDATE counts SET
                            debtor = ?3, 
                            title = ?4, 
                            description = ?5, 
                            value = ?6,  
                            paid_installments = ?7, 
                            installments = ?8, 
                            date_in = strftime('%Y-%m-%d', ?9), 
                            date_out = strftime('%Y-%m-%d', ?10), 
                            status = ?11, 
                            nature = ?12
                            WHERE count_id = ?1 AND user_id = ?2",
                        )
                        .unwrap()
                        .execute(params![
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
                        ])
                        .map_err(|_| {
                            DataBaseError::EditCountsError(ErrorLog {
                                title: "Error to edit count",
                                code: 808,
                                file: "Database.rs",
                            })
                        })?;
                }

                Ok(())
            }
            Data::UserDb(user) => {
                let mut stmt = self
                    .pool
                    .prepare(
                        "UPDATE users SET
                            password = ?1
                            WHERE user_id = ?2",
                    )
                    .unwrap();

                stmt.execute(params![user.password, user.id]).unwrap();

                Ok(())
            }
            Data::LastLogin(id) => {
                let mut stmt = self
                    .pool
                    .prepare(
                        "UPDATE users SET
                            last_login = CURRENT_DATE
                            WHERE user_id = ?1",
                    )
                    .unwrap();

                stmt.execute(params![id]).unwrap();

                Ok(())
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })),
        }
    }

    pub async fn delete(&self, data: Data) -> Result<(), DataBaseError> {
        match data {
            Data::UserDb(user) => {
                self.pool
                    .prepare("DELETE FROM users WHERE user_id = ?1")
                    .unwrap()
                    .execute(params![user.id])
                    .map_err(|_| {
                        DataBaseError::DeleteUserError(ErrorLog {
                            title: "User not found!",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                Ok(())
            }
            Data::Count(count_id, user_id) => {
                println!("Deletando conta {}{}...", user_id, count_id);

                let _ = self
                    .pool
                    .prepare("DELETE FROM counts WHERE user_id = ?1 AND count_id = ?2")
                    .unwrap()
                    .execute(params![user_id, format!("{}{}", user_id, count_id)])
                    .map_err(|_| {
                        DataBaseError::DeleteCountError(ErrorLog {
                            title: "Error on delete count",
                            code: 500,
                            file: "Database.rs",
                        })
                    })?;

                Ok(())
            }
            _ => Err(DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to delete",
                code: 817,
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Data {
    NewUser(NewUser),
    User(User),
    UserDb(UserDb),
    Counts(ListCount, UserDb, i16),
    Years(ListCount, UserDb),
    Count(i32, i32),
    LastLogin(i32),
}
