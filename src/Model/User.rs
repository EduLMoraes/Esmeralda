#[derive(PartialEq, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    pub fn is_empty(&self) -> bool {
        self.username.is_empty() || self.password.is_empty() || self.email.is_empty()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserDb {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub last_login: String,
}
