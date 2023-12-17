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

pub struct Info{
    pub debtor: String,
    pub value: f64,
    pub date_in: String,
    pub date_out: String,
    pub installments: String,
    pub status: bool
}