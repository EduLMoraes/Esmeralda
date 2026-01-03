use super::{UserService, UserServiceError};
use chrono::Utc;
use esmeralda_apis::mailjet::MailjetApi;
use esmeralda_cryptography::PasswordHasher;
use esmeralda_database::Database;
use esmeralda_entities::user::User;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserServiceImpl<D, H, M>
where
    D: Database<User>,
    H: PasswordHasher,
    M: MailjetApi,
{
    user_repo: Arc<D>,
    hasher: Arc<H>,
    mail_api: Arc<M>,
}

impl<D, H, M> UserServiceImpl<D, H, M>
where
    D: Database<User>,
    H: PasswordHasher,
    M: MailjetApi,
{
    pub fn new(user_repo: Arc<D>, hasher: Arc<H>, mail_api: Arc<M>) -> Self {
        Self {
            user_repo,
            hasher,
            mail_api,
        }
    }
}

impl<D, H, M> UserService for UserServiceImpl<D, H, M>
where
    D: Database<User>,
    H: PasswordHasher,
    M: MailjetApi,
{
    fn login(&self, email: &str, password: &str) -> Result<User, UserServiceError> {
        let user = self
            .user_repo
            .get_by_email(email)
            .map_err(|_| UserServiceError::UserNotFound)?;

        let valid = self
            .hasher
            .verify_password(password, &user.password)
            .map_err(|e| UserServiceError::Cryptography(e.to_string()))?;

        if valid {
            let mut user = user;
            user.last_login = Utc::now().naive_utc().date();
            self.user_repo
                .edit(user.clone())
                .map_err(|e| UserServiceError::Database(e.to_string()))?;
            Ok(user)
        } else {
            Err(UserServiceError::InvalidCredentials)
        }
    }

    fn add_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, UserServiceError> {
        let hashed_password = self
            .hasher
            .hash_password(password)
            .map_err(|e| UserServiceError::Cryptography(e.to_string()))?;

        let new_user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password: hashed_password,
            last_login: Utc::now().naive_utc().date(),
        };

        self.user_repo
            .insert(new_user.clone())
            .map_err(|e| UserServiceError::Database(e.to_string()))?;
        Ok(new_user)
    }

    fn edit_user(&self, user: &User) -> Result<(), UserServiceError> {
        self.user_repo
            .edit(user.clone())
            .map_err(|e| UserServiceError::Database(e.to_string()))
    }

    fn restore_password(&self, email: &str) -> Result<(), UserServiceError> {
        let mut user = self
            .user_repo
            .get_by_email(email)
            .map_err(|_| UserServiceError::UserNotFound)?;

        // This should be a random string, but for now I'll hardcode it.
        // I should have migrated the gen_string function.
        let new_password = "new_password";

        user.password = self
            .hasher
            .hash_password(new_password)
            .map_err(|e| UserServiceError::Cryptography(e.to_string()))?;

        self.user_repo
            .edit(user.clone())
            .map_err(|e| UserServiceError::Database(e.to_string()))?;

        self.mail_api
            .send_email(
                "esmeralda.restorepass@gmail.com",
                email,
                "Password recovery",
                format!("Your new password is: {}", new_password),
                "Password recovery".to_string(),
            )
            .map_err(|e| UserServiceError::Api(e.to_string()))
    }
}

