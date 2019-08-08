use crate::Error;

/// A Result type for handling errors from mire
pub type Result<T> = std::result::Result<T, Error>;
