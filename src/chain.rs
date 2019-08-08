use std::error::Error as StdError;
use std::iter::FusedIterator;

/// Iterator over the chain of source errors
#[derive(Clone)]
pub struct Chain<'a> {
    current: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn StdError + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let error = self.current;
        self.current = self.current.and_then(StdError::source);
        error
    }
}

impl<'a> FusedIterator for Chain<'a> {}

/// Extension trait for error chaining
pub trait ChainExt {
    /// Returns an iterator over the chain of source errors
    fn chain(&self) -> Chain<'_>;
}

impl<'a, T: StdError + 'a + 'static> ChainExt for T {
    fn chain(&self) -> Chain<'_> {
        Chain {
            current: Some(self),
        }
    }
}
