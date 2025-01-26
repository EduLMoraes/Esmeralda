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

impl UserDb {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_last_login(&self) -> &str {
        &self.last_login
    }
}
