use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    GenericError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}