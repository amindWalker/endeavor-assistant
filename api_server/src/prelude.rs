pub use crate::error::LibError;

// Replacing the std/core Result Error with miette Result and custom LibError
pub type Result<T> = miette::Result<T, LibError>;

// Generic newtype pattern wrapper for convenience
pub struct W<T>(pub T);