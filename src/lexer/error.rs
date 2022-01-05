
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::error::Error;

#[derive(Debug)]
pub(crate) enum LexError {
    InvalidChar { chr: char, pos: usize },
}

impl Display for LexError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Debug::fmt(self, fmt)
    }
}

impl Error for LexError {    }
