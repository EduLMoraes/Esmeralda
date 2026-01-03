use chrono::NaiveDate;
use esmeralda_database::{people_repository::PeopleRepository, rusqlite_impl::Db, Database};
use esmeralda_entities::people::People;
use uuid::Uuid;

fn setup() -> (PeopleRepository, People) {
    let db = Db::new().unwrap();
    let people_repo = PeopleRepository::new(db);
    let person = People {
        id: Uuid::new_v4(),
        name: "John".to_string(),
        surname: Some("Doe".to_string()),
        birthday: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        ..Default::default()
    };
    (people_repo, person)
}

#[test]
fn test_insert_and_get_person() {
    let (people_repo, person) = setup();

    people_repo.insert(person.clone()).unwrap();

    let fetched_person = people_repo.get_data(&person.id.to_string()).unwrap();
    assert_eq!(person, fetched_person);
}

#[test]
fn test_edit_person() {
    let (people_repo, mut person) = setup();

    people_repo.insert(person.clone()).unwrap();

    person.name = "Jane".to_string();
    people_repo.edit(person.clone()).unwrap();

    let fetched_person = people_repo.get_data(&person.id.to_string()).unwrap();
    assert_eq!(person, fetched_person);
}

#[test]
fn test_suspend_person() {
    let (people_repo, person) = setup();

    people_repo.insert(person.clone()).unwrap();

    people_repo.suspend(person.clone()).unwrap();

    let result = people_repo.get_data(&person.id.to_string());
    assert!(result.is_err());
}
