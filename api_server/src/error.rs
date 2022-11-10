use core::fmt;
use std::error::Error;

pub struct BoxedError(pub Box<dyn Error + Send + Sync>);

impl fmt::Debug for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for BoxedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.0.description()
    }
    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        self.0.cause()
    }
}

// Define errors and diagnostics here
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[rustfmt::skip]
pub enum LibError<ErrType: 'static + Error = BoxedError> {
    #[error("Error: {0}")]
    GenericError(#[source] ErrType),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}