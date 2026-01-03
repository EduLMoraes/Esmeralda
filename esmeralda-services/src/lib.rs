use esmeralda_entities::user::User;
use thiserror::Error;

pub mod user_service;
pub use user_service::UserServiceImpl;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error: {0}")]
    Database(String),
    #[error("Cryptography error: {0}")]
    Cryptography(String),
    #[error("API error: {0}")]
    Api(String),
}

pub trait UserService {
    fn login(&self, email: &str, password: &str) -> Result<User, UserServiceError>;
    fn add_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, UserServiceError>;
    fn edit_user(&self, user: &User) -> Result<(), UserServiceError>;
    fn restore_password(&self, email: &str) -> Result<(), UserServiceError>;
}
