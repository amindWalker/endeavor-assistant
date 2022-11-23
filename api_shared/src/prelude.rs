use std::fmt::{self, Debug, Display};

pub use crate::error::LibError;

// Generic newtype pattern wrapper for conveniently implementing
// external `Traits` on external `Types` living in different crates
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub struct UseTrait<T: std::fmt::Debug>(pub T);

impl<T: std::fmt::Debug> Display for UseTrait<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
