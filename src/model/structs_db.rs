pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser{
    pub fn is_empty(&self) -> bool{
        if self.username.is_empty() || self.password.is_empty() || self.email.is_empty(){
            true
        }else{
            false
        }
    }
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
