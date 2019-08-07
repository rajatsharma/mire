use crate::Error;

/// A Result type for handling errors from this library
pub type Result<T> = std::result::Result<T, Error>;
