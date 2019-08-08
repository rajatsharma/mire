use std::any::TypeId;
use std::backtrace::Backtrace;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::sync::Arc;

use crate::chain::ChainExt;
use crate::context::Context;

/// The core Error type of the library
pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    source: Option<Box<dyn StdError + Send + Sync>>,
    context: Option<Box<dyn Context + Send + Sync>>,
    backtrace: Option<Backtrace>,
    type_id: TypeId,
}

impl Error {
    /// Create a new Error from any error type that implements StdError + Send + Sync + 'static
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let backtrace = if std::env::var_os("RUST_BACKTRACE").is_some() {
            Some(Backtrace::capture())
        } else {
            None
        };

        Error {
            inner: Box::new(ErrorImpl {
                source: Some(Box::new(error)),
                context: None,
                backtrace,
                type_id: TypeId::of::<E>(),
            }),
        }
    }

    /// Create a new Error with just a message
    pub fn msg<M>(message: M) -> Self
    where
        M: Display + Send + Sync + 'static,
    {
        #[derive(Debug)]
        struct StringError(Arc<str>);

        impl Display for StringError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl StdError for StringError {}

        let string = message.to_string();
        Error::new(StringError(Arc::from(string)))
    }

    /// Get the backtrace if available
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace.as_ref()
    }

    /// Add context to the error
    pub fn context<C>(mut self, context: C) -> Self
    where
        C: Context + Send + Sync + 'static,
    {
        self.inner.context = Some(Box::new(context));
        self
    }

    /// Attempt to downcast the error to a concrete type
    pub fn downcast<E: StdError + Send + Sync + 'static>(self) -> Result<E, Self> {
        if self.inner.type_id == TypeId::of::<E>() {
            if let Some(source) = self.inner.source {
                let source = Box::into_raw(source) as *mut dyn StdError;
                // SAFETY: We've verified the type_id matches E
                let error = unsafe { Box::from_raw(source as *mut E) };
                return Ok(*error);
            }
        }
        Err(self)
    }

    /// Attempt to downcast a reference to the error to a concrete type
    pub fn downcast_ref<E: StdError + Send + Sync + 'static>(&self) -> Option<&E> {
        if self.inner.type_id == TypeId::of::<E>() {
            if let Some(source) = &self.inner.source {
                let ptr = source.as_ref() as *const dyn StdError;
                // SAFETY: We've verified the type_id matches E
                return unsafe { Some(&*(ptr as *const E)) };
            }
        }
        None
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return f
                .debug_struct("Error")
                .field("source", &self.inner.source)
                .field("context", &self.inner.context)
                .field("backtrace", &self.inner.backtrace)
                .finish();
        }

        write!(f, "{}", self)?;

        if let Some(ctx) = &self.inner.context {
            write!(f, "\nContext: {}", ctx)?;
        }

        if let Some(src) = self.chain().skip(1).next() {
            write!(f, "\nCaused by: {}", src)?;
        }

        if let Some(bt) = &self.inner.backtrace {
            write!(f, "\n{}", bt)?;
        }

        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.inner.source {
            Display::fmt(source, f)
        } else {
            write!(f, "Unknown error")
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn StdError + 'static))
    }
}

/// Extension trait for Error type
pub trait ErrorExt {
    /// Add context to an error
    fn with_context<C, F>(self, f: F) -> Self
    where
        C: Context + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl ErrorExt for Error {
    fn with_context<C, F>(self, f: F) -> Self
    where
        C: Context + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.context(f())
    }
}
