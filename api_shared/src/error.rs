use std::{error::Error, fmt};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// # `Error` Library
///
/// This library helps to diagnose and implement errors
/// using `miette` and `thiserror`

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub struct BoxedError(pub Box<dyn Error + Send + Sync>);

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
// Define errors and diagnostics
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum LibError {
    #[error(transparent)]
    #[diagnostic(
        code(LibError::IOError),
        help("Cheque se os campos digitados são compatíveis")
    )]
    IOError(#[from] std::io::Error),
    #[error("Erro: email já cadastrado")]
    #[diagnostic(code(LibError::EmailTaken), help("Tente um email diferente"))]
    EmailTaken,
    #[error("Erro: nome de usuário já cadastrado")]
    #[diagnostic(
        code(LibError::UserTaken),
        help("Tente um nome de usuário diferente")
    )]
    UserTaken,
    #[error("Erro: senha inválida")]
    #[diagnostic(
        code(LibError::UserTaken),
        help("Senhas devem conter no mínimo 8 caracteres")
    )]
    PasswordInvalid,
    #[error("Erro: erro desconhecido")]
    #[diagnostic(code(LibError::UnknownError), help("Cheque o código fonte"))]
    UnknownError,
}

// implementing Axum IntoResponse for custom errors
impl IntoResponse for LibError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::IOError(err) => format!(
                "Dados enviados estão incorretos.\n\nErro interno: {err}"
            ),
            Self::EmailTaken => "Email já cadastrado".into(),
            Self::UserTaken => "Usuário já cadastrado".into(),
            Self::PasswordInvalid => {
                "Senhas devem conter no mínimo 8 caracteres".into()
            }
            Self::UnknownError => "Erro desconhecido do servidor".into(),
        };

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl From<LibError> for Response {
    fn from(error: LibError) -> Self {
        use LibError::*;
        match error {
            IOError(err) => IOError(err).into(),
            EmailTaken => EmailTaken.into(),
            UserTaken => UserTaken.into(),
            PasswordInvalid => PasswordInvalid.into(),
            _ => UnknownError.into(),
        }
    }
}
