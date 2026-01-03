use chrono::NaiveDate;
use esmeralda_database::{rusqlite_impl::Db, user_repository::UserRepository, Database};
use esmeralda_entities::user::User;
use uuid::Uuid;

fn setup() -> (UserRepository, User) {
    let db = Db::new().unwrap();
    let user_repo = UserRepository::new(db);
    let user = User {
        id: Uuid::new_v4(),
        last_login: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        email: "test@test.com".to_string(),
        password: "test_password".to_string(),
        username: "test_user".to_string(),
    };
    (user_repo, user)
}

#[test]
fn test_insert_and_get_user() {
    let (user_repo, user) = setup();

    // Insert user
    user_repo.insert(user.clone()).unwrap();

    // Get user
    let fetched_user = user_repo.get_data(&user.id.to_string()).unwrap();
    assert_eq!(user.id, fetched_user.id);
    assert_eq!(user.username, fetched_user.username);
    assert_eq!(user.email, fetched_user.email);
    assert_eq!(user.last_login, fetched_user.last_login);
}

#[test]
fn test_get_user_by_email() {
    let (user_repo, user) = setup();

    user_repo.insert(user.clone()).unwrap();

    let fetched_user = user_repo.get_by_email(&user.email).unwrap();
    assert_eq!(user.id, fetched_user.id);
    assert_eq!(user.username, fetched_user.username);
    assert_eq!(user.email, fetched_user.email);
    assert_eq!(user.last_login, fetched_user.last_login);
}

#[test]
fn test_edit_user() {
    let (user_repo, mut user) = setup();

    user_repo.insert(user.clone()).unwrap();

    // Modify user
    user.username = "new_username".to_string();
    user_repo.edit(user.clone()).unwrap();

    let fetched_user = user_repo.get_data(&user.id.to_string()).unwrap();
    assert_eq!(user.id, fetched_user.id);
    assert_eq!(user.username, fetched_user.username);
    assert_eq!(user.email, fetched_user.email);
    assert_eq!(user.last_login, fetched_user.last_login);
}

#[test]
fn test_suspend_user() {
    let (user_repo, user) = setup();

    user_repo.insert(user.clone()).unwrap();

    // Suspend user
    user_repo.suspend(user.clone()).unwrap();

    // Try to get suspended user
    let result = user_repo.get_data(&user.id.to_string());
    assert!(result.is_err());
}
