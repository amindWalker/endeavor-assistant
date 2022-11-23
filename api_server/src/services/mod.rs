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

#[cfg(test)]
mod tests {
    use crate::services::{mock_db_user, UserModel};

    fn mock_existing_user() -> UserModel {
        UserModel {
            email: "user@email.com".to_string(),
            password: "valid_password".to_string(),
            username: "username".to_string(),
        }
    }
    fn mock_new_user() -> UserModel {
        UserModel {
            email: "new_user@email.com".to_string(),
            password: "invalid".to_string(),
            username: "new_username".to_string(),
        }
    }

    mod test_create_user_service {
        use super::{mock_db_user, mock_existing_user};
        use crate::services::{tests::mock_new_user, UserModel};

        #[test]
        fn test_email_taken() -> miette::Result<()> {
            let is_email_taken = mock_existing_user()
                .email
                .eq(&mock_db_user().email);

            miette::ensure!(is_email_taken, "Error: email already used");
            Ok(())
        }

        #[test]
        fn test_email_available() -> miette::Result<()> {
            let is_email_available = mock_new_user()
                .email
                .ne(&mock_db_user().email);

            miette::ensure!(is_email_available, "Success!");
            Ok(())
        }

        #[test]
        fn test_username_taken() -> miette::Result<()> {
            let is_username_taken = mock_existing_user()
                .username
                .eq(&mock_db_user().username);

            miette::ensure!(is_username_taken, "Error: username already used");
            Ok(())
        }

        #[test]
        fn test_username_available() -> miette::Result<()> {
            let is_username_taken = mock_new_user()
                .username
                .ne(&mock_db_user().username);

            miette::ensure!(is_username_taken, "Success!");
            Ok(())
        }

        #[test]
        fn test_password_invalid() -> miette::Result<()> {
            let is_password_invalid = mock_new_user().password.len() < 8;

            miette::ensure!(
                is_password_invalid,
                "Error: password must be at least 8 characters long"
            );
            Ok(())
        }

        #[test]
        fn test_password_valid() -> miette::Result<()> {
            let valid_password = UserModel {
                password: "valid_password".to_string(),
                ..mock_new_user()
            };
            let is_password_valid = valid_password.password.len() >= 8;

            miette::ensure!(is_password_valid, "Success!");
            Ok(())
        }
    }
}
