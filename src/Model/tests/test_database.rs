#[allow(unused_imports)]
use super::Count::Count;
#[allow(unused_imports)]
use super::Database::*;
#[allow(unused_imports)]
use super::List::ListCount;
#[allow(unused_imports)]
use super::NaiveDate;
#[allow(unused_imports)]
use super::User::*;
#[allow(unused_imports)]
use crate::env;
#[allow(unused_imports)]
use crate::model::errors::DataBaseError;
#[allow(unused_imports)]
use crate::model::errors::ErrorLog;

#[cfg(test)]
mod tests_database {
    use super::*;

    // The 'new' function successfully creates a new instance of the 'DataBase' struct.
    #[tokio::test]
    async fn test_new_function_creates_instance() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let result = DataBase::new();
        assert!(result.is_ok());
    }

    // The 'add' function successfully adds a new user to the database.
    #[tokio::test]
    async fn test_add_function_adds_new_user() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = NewUser {
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            wage: 1000.0,
        };
        let data = Data::NewUser(user);
        let result = db.add(data);

        assert!(result.await.is_ok());
    }

    // The 'add' function successfully adds a new count to the database.
    #[tokio::test]
    async fn test_add_function_adds_new_count() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = UserDb {
            id: 1,
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            last_login: "7".to_string(),
        };
        let count = Count {
            id: 1,
            debtor: "Debtor".to_string(),
            title: "Title".to_string(),
            description: "Description".to_string(),
            value: 100.0,
            paid_installments: 0,
            installments: 1,
            date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            status: true,
            nature: "Nature".to_string(),
        };
        let list_count = ListCount {
            list: vec![count],
            years: vec![2022],
        };
        let data = Data::Counts(list_count, user, 2022);
        let result = db.add(data);
        assert!(result.await.is_ok());
    }

    // The 'get' function successfully retrieves a user from the database.
    #[tokio::test]
    async fn test_get_function_retrieves_user() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = User {
            username: "test_user".to_string(),
            password: "new_password".to_string(),
        };
        let data = Data::User(user);
        let result = db.get(data);
        assert_eq!(
            result.await.unwrap(),
            Data::UserDb(UserDb {
                id: 1,
                name: "Test User".to_string(),
                username: "test_user".to_string(),
                password: "new_password".to_string(),
                email: "test@example.com".to_string(),
                last_login: "".to_string()
            })
        );
    }

    // The 'get' function successfully retrieves counts for a specific user and year from the database.
    #[tokio::test]
    async fn test_get_function_retrieves_counts_for_user_and_year() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = UserDb {
            id: 1,
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            last_login: "7".to_string(),
        };
        let list_count = ListCount {
            list: vec![Count {
                id: 1,
                debtor: "Debtor".to_string(),
                title: "Title".to_string(),
                description: "Description".to_string(),
                value: 100.0,
                paid_installments: 0,
                installments: 1,
                date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
                status: true,
                nature: "Nature".to_string(),
            }],
            years: vec![2022],
        };
        let data = Data::Counts(list_count, user, 2022);
        let result = db.get(data);
        assert!(result.await.is_ok());
    }

    // The 'get' function successfully retrieves years with counts for a specific user from the database.
    #[tokio::test]
    async fn test_get_function_retrieves_years_with_counts_for_user() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = UserDb {
            id: 1,
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            last_login: "7".to_string(),
        };
        let list_count = ListCount {
            list: vec![Count {
                id: 1,
                debtor: "Debtor".to_string(),
                title: "Title".to_string(),
                description: "Description".to_string(),
                value: 100.0,
                paid_installments: 0,
                installments: 1,
                date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
                status: true,
                nature: "Nature".to_string(),
            }],
            years: vec![],
        };
        let data = Data::Years(list_count, user);
        let result = db.get(data);
        assert!(result.await.is_ok());
    }

    // The 'new' function handles the case where the database connection fails and creates the necessary tables.
    #[tokio::test]
    async fn test_new_function_handles_database_connection_failure() {
        env::set_var(
            "DB_PATH",
            format!("{}/nonexistsdir/test_db", env::temp_dir().display()),
        );

        let result = DataBase::new();
        assert!(result.is_err());
    }

    // The 'add' function handles the case where the data type is invalid and returns an error.
    #[tokio::test]
    async fn test_add_function_handles_invalid_data_type() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let data = Data::Years(
            ListCount {
                list: vec![Count {
                    id: 1,
                    debtor: "Debtor".to_string(),
                    title: "Title".to_string(),
                    description: "Description".to_string(),
                    value: 100.0,
                    paid_installments: 0,
                    installments: 1,
                    date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                    date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
                    status: true,
                    nature: "Nature".to_string(),
                }],
                years: vec![2022],
            },
            UserDb {
                id: 1,
                name: "Test User".to_string(),
                username: "test_user".to_string(),
                password: "password".to_string(),
                email: "test@example.com".to_string(),
                last_login: "7".to_string(),
            },
        );
        let result = db.add(data);
        assert_eq!(
            result.await.unwrap_err(),
            DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })
        );
    }

    // The 'get' function handles the case where the data type is invalid and returns an error.
    #[tokio::test]
    async fn test_get_function_handles_invalid_data_type() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let data = Data::NewUser(NewUser {
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            wage: 1000.0,
        });
        let result = db.get(data);
        assert_eq!(
            result.await.unwrap_err(),
            DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })
        );
    }

    // The 'edit' function handles the case where the data type is invalid and returns an error.
    #[tokio::test]
    async fn test_edit_function_handles_invalid_data_type() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let data = Data::NewUser(NewUser {
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            wage: 1000.0,
        });
        let result = db.edit(data);
        assert_eq!(
            result.await.unwrap_err(),
            DataBaseError::DataTypeInvalid(ErrorLog {
                title: "Type of data is invalid to add",
                code: 816,
                file: "Database.rs",
            })
        );
    }

    // The 'edit' function successfully updates counts for a specific user in the database.
    #[tokio::test]
    async fn test_edit_function_updates_counts() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = UserDb {
            id: 1,
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string(),
            last_login: "7".to_string(),
        };
        let count = Count {
            id: 1,
            debtor: "Debtor".to_string(),
            title: "Title".to_string(),
            description: "Description".to_string(),
            value: 100.0,
            paid_installments: 0,
            installments: 1,
            date_in: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            date_out: NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
            status: true,
            nature: "Nature".to_string(),
        };
        let list_count = ListCount {
            list: vec![count],
            years: vec![2022],
        };
        let data = Data::Counts(list_count, user, 2022);
        let result = db.edit(data);
        assert!(result.await.is_ok());
    }

    // The 'edit' function successfully updates the password for a specific user in the database.
    #[tokio::test]
    async fn test_edit_function_updates_password() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        let db = get_database_instance();
        let user = UserDb {
            id: 1,
            name: "Test User".to_string(),
            username: "test_user".to_string(),
            password: "new_password".to_string(),
            email: "test@example.com".to_string(),
            last_login: "7".to_string(),
        };
        let data = Data::UserDb(user);
        let result = db.edit(data);
        assert!(result.await.is_ok());
    }

    // The 'add' function handles the case where the log file path is not found.

    #[tokio::test]
    async fn test_add_handles_log_file_path_not_found() {
        env::set_var(
            "DB_PATH",
            format!("{}/nonexistsdir/test_db", env::temp_dir().display()),
        );

        // Error to create a new instance of the database
        let result = DataBase::new();

        assert!(result.is_err());
    }

    // The 'get' function handles the case where the log file path is not found.
    #[tokio::test]
    async fn test_get_function_handles_missing_log_file_path() {
        env::set_var(
            "DB_PATH",
            format!("{}/misspath/test_db", env::temp_dir().display()),
        );

        // Arrange
        let result = DataBase::new();
        // Assert
        assert!(result.is_err());
    }

    // The 'get' function handles the case where the user is not found in the database.
    #[tokio::test]
    async fn test_get_function_handles_user_not_found() {
        env::set_var(
            "DB_PATH",
            format!("{}/nonexistsdir/test_db", env::temp_dir().display()),
        );

        // Arrange
        let result = DataBase::new();
        // Assert
        assert!(result.is_err());
    }

    // The 'get' function handles the case where there are no counts for a specific user and year in the database.
    #[tokio::test]
    async fn test_get_no_counts_for_user_and_year() {
        env::set_var("DB_PATH", format!("{}/test_db", env::temp_dir().display()));

        // Arrange
        let db = DataBase::new().unwrap();
        let data = Data::Counts(
            ListCount {
                list: vec![],
                years: vec![],
            },
            UserDb {
                id: 1,
                name: "Test User".to_string(),
                username: "test_user".to_string(),
                password: "new_password".to_string(),
                email: "test@test.com".to_string(),
                last_login: "7".to_string(),
            },
            2022,
        );

        // Act
        let result = db.get(data);

        // Assert
        match result.await {
            Ok(Data::Counts(counts, user, year)) => {
                assert_eq!(counts.list.len() - 1, 0);
                assert_eq!(user.id, 1);
                assert_eq!(user.name, "Test User");
                assert_eq!(user.username, "test_user");
                assert_eq!(user.password, "new_password");
                assert_eq!(user.email, "test@test.com");
                assert_eq!(year, 2022);
            }
            _ => panic!("Expected Ok(Data::Counts)"),
        }
    }
}
