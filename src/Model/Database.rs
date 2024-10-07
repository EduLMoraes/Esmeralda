use crate::prelude::env::var;
use crate::prelude::log;
use crate::prelude::model::{
    errors::{DataBaseError, ErrorLog},
    Count::Count,
    List::ListCount,
    User::*,
};
use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use std::env;
use std::sync::Mutex;

#[allow(dead_code)]
#[derive(Debug)]
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
                    use std::process::Command;
                    let response = Command::new(env::var("MANAGER_PATH").unwrap())
                        .arg("create")
                        .arg(format!("--path={}", env::var("DB_PATH")
                            .unwrap())
                        )
                        .output()
                        .expect("erro ao rodar script");

                    // match response {
                    //     Ok(_) => {}
                    //     Err(e) => println!("Erro ao executar manager_db: {e:?}"),
                    // };
                    
                    let conn = Connection::open(var("DB_PATH").unwrap()).map_err(|_| {
                        DataBaseError::CreatePoolError(ErrorLog {
                            title: "Error to connect database",
                            code: 812,
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
                let mut stmt = self
                    .pool
                    .prepare("INSERT INTO Users (username, password, email) VALUES (?1, ?2, ?3) ")
                    .unwrap();

                stmt.execute([
                    user.username,
                    user.password,
                    user.email,
                    // user.name,
                    // user.wage.to_string(),
                ])
                .map_err(|err| {
                    let _ = log(path.clone().into(), format!("{err:?}").trim());
                    DataBaseError::AddUserError(ErrorLog {
                        title: "User already existis",
                        code: 500,
                        file: "Database.rs",
                    })
                })?;

                let mut stmt = self
                    .pool
                    .prepare("INSERT INTO People (name) VALUES (?1) ")
                    .unwrap();

                stmt.execute([
                    user.name,
                ])
                .map_err(|err| {
                    let _ = log(path.into(), format!("{err:?}").trim());
                    DataBaseError::AddUserError(ErrorLog {
                        title: "Error to add people",
                        code: 500,
                        file: "Database.rs",
                    })
                })?;

                Ok(())
            }
            Data::Counts(counts, user, _year) => {
                self.pool
                    .prepare(
                        "INSERT INTO Counts (
                        id_count,
                        id_user,
                        debtor, 
                        title, 
                        description, 
                        value, 
                        paid_installments, 
                        installments, 
                        date_in, 
                        date_out, 
                        nature
                    ) VALUES (
                        (SELECT COALESCE(MAX(id_count), 0)+1 AS id_count2 FROM counts where id_user = ?1),
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
                    strftime('%Y-%m-%d', ?8), strftime('%Y-%m-%d', ?9), 
                    ?10) ",
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
                    .prepare("SELECT id_user, username, name, password, email, coalesce(strftime('%m', last_login), '0') as last_login FROM users natural join people WHERE username = ?1 LIMIT 1")
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
                                .get::<_, i32>("id_user")
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
                        "SELECT id_count, id_user,
                        strftime('%Y-%m-%d', date_in) AS date_in, strftime('%Y-%m-%d', date_out) AS date_out, 
                        debtor, title, description, value, paid_installments, installments, nature
                        FROM Counts
                        WHERE id_user = ?1
                        AND
                        ( strftime('%Y', date_in) like ?2
                        OR strftime('%Y', date_out) like ?2 )
                        order by id_count",
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

                while let Ok(Some(row)) = rows.next() {
                    let nature = row
                        .get::<_, String>("nature")
                        .map_err(|_| {
                            DataBaseError::GetCountsError(ErrorLog {
                                title: "Error to get nature value",
                                code: 500,
                                file: "Database.rs",
                            })
                        })
                        .unwrap();

                    let count = Count {
                        id: row
                            .get::<_, i32>("id_count")
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
                        status: row.get::<_, i64>("installments").map_err(|err| {
                            println!("{:?}", err);
                            DataBaseError::GetCountsError(ErrorLog {
                                title: "Error to get status value",
                                code: 500,
                                file: "Database.rs",
                            })
                        })? == row.get::<_, i64>("paid_installments").map_err(|err| {
                            println!("{:?}", err);
                            DataBaseError::GetCountsError(ErrorLog {
                                title: "Error to get status value",
                                code: 500,
                                file: "Database.rs",
                            })
                        })?,
                        nature: if nature.trim().is_empty() {
                            String::from("Outros")
                        } else {
                            nature
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
                        DISTINCT strftime('%Y', date_out) as years
                        FROM counts 
                        WHERE
                        id_user = ?1
                        UNION
                        SELECT
                        DISTINCT strftime('%Y', date_in) as years
                        FROM counts
                        WHERE
                        id_user = ?1",
                    )
                    .unwrap();

                let mut rows = stmt.query([user.id as i64]).unwrap();

                let mut years: Vec<i16> = Vec::new();

                while let Ok(Some(row)) = rows.next() {
                    years.push(
                        row.get::<_, String>("years")
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
            Data::Groups(mut groups, id_user) => {
                let mut stmt = self.pool.prepare("
                        SELECT nature FROM counts WHERE id_user = ?1 GROUP BY nature ORDER BY nature;
                    ")
                    .map_err(|_|{
                    DataBaseError::GetCountsError(ErrorLog { title: "Error to get groups", code: 500, file: "Database.rs" })  
                    })?;

                let mut rows = stmt.query(params![&id_user]).map_err(|_| {
                    DataBaseError::GetCountsError(ErrorLog {
                        title: "Error to get groups of natures",
                        code: 500,
                        file: "Database.rs",
                    })
                })?;

                while let Ok(Some(row)) = rows.next() {
                    let value_row = row.get::<_, String>("nature").map_err(|_| {
                        DataBaseError::GetCountsError(ErrorLog {
                            title: "Error to get nature value on groups",
                            code: 804,
                            file: "Database.rs",
                        })
                    })?;

                    if !groups.contains(&value_row) && !value_row.trim().is_empty() {
                        groups.push(value_row);
                    }
                }

                Ok(Data::Groups(groups, id_user))
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
                            nature = ?11
                            WHERE id_count = ?1 AND id_user = ?2",
                        )
                        .unwrap()
                        .execute(params![
                            &counts.list[i].id,
                            &user.id,
                            &counts.list[i].debtor,
                            &counts.list[i].title,
                            &counts.list[i].description,
                            &counts.list[i].value,
                            &(counts.list[i].paid_installments as i32),
                            &(counts.list[i].installments as i32),
                            &counts.list[i].date_in.to_string(),
                            &counts.list[i].date_out.to_string(),
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
                            WHERE id_user = ?2",
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
                            WHERE id_user = ?1",
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
                    .prepare("DELETE FROM users WHERE id_user = ?1")
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
            Data::Count(id_count, id_user) => {
                println!("Deletando conta {}{}...", id_user, id_count);

                let _ = self
                    .pool
                    .prepare("DELETE FROM counts WHERE id_user = ?1 AND id_count = ?2")
                    .unwrap()
                    .execute(params![id_user, format!("{}{}", id_user, id_count)])
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
    Groups(Vec<String>, i32),
}
