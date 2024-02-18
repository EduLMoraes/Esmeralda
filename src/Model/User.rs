/// Represents a new user with a username, password, and email.
#[derive(PartialEq, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    /// Checks if any of the fields (username, password, or email) are empty.
    ///
    /// # Returns
    ///
    /// * `true` if any of the fields are empty.
    /// * `false` if none of the fields are empty.
    pub fn is_empty(&self) -> bool {
        if self.username.is_empty() || self.password.is_empty() || self.email.is_empty() {
            true
        } else {
            false
        }
    }
}

/// Represents a user with a username and password.
#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

/// Represents a user in a database.
#[derive(Clone, PartialEq, Debug)]
pub struct UserDb {
    pub id: i32,
    pub username: String,
    pub password: String,
}
