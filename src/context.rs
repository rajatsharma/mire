use crate::result::Result;
use std::fmt::{Debug, Display};

/// Trait to represent error context
pub trait Context: Display + Debug {}

impl Context for String {}
impl Context for &str {}

/// Extension trait for Result types to add context
pub trait ContextExt<T, E> {
    /// Add context to an error
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Context + Send + Sync + 'static;

    /// Add context to an error using a closure
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Context + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ContextExt<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Context + Send + Sync + 'static,
    {
        self.map_err(|error| crate::Error::new(error).context(context))
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Context + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| crate::Error::new(error).context(f()))
    }
}
