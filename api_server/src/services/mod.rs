// external crates
use serde::Deserialize;
use std::fmt;
// local modules
use api_shared::prelude::LibError;

#[derive(Debug, Deserialize)]
pub struct UserModel {
    pub email: String,
    pub password: String,
    pub username: String,
}

// TODO: replace mock data with PostgreSQL
pub fn mock_db_user() -> UserModel {
    UserModel {
        email: "user@email.com".to_string(),
        password: "valid_password".to_string(),
        username: "username".to_string(),
    }
}

impl UserModel {
    pub fn create_user_service(self) -> Result<Self, LibError> {
        let is_email_taken = self.email.eq(&mock_db_user().email);
        if is_email_taken {
            return Err(LibError::EmailTaken);
        }
        let is_username_taken = self
            .username
            .eq(&mock_db_user().username);
        if is_username_taken {
            return Err(LibError::UserTaken);
        }
        let is_password_invalid = self.password.len() < 7;
        if is_password_invalid {
            return Err(LibError::PasswordInvalid);
        }

        Ok(Self {
            email: self.email,
            password: self.password,
            username: self.username,
        })
    }
}

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
