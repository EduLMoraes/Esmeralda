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

#[async_trait::async_trait]
pub trait UserService {
    async fn login(&self, email: &str, password: &str) -> Result<User, UserServiceError>;
    async fn add_user(&self, username: &str, email: &str, password: &str) -> Result<User, UserServiceError>;
    async fn edit_user(&self, user: &User) -> Result<(), UserServiceError>;
    async fn restore_password(&self, email: &str) -> Result<(), UserServiceError>;
}