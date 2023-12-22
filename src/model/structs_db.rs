pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone)]
pub struct User{
    pub username: String,
    pub password: String
}

pub struct UserDb{
    pub id: i32,
    pub username: String,
    pub password: String
}
