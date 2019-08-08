mod chain;
mod context;
mod error;
mod result;

pub use result::Result;

#[macro_use]
mod macros;

pub mod prelude {
    pub use crate::{ChainExt, Context, ContextExt, Error, Result};
    pub use std::result::Result as StdResult;
}
