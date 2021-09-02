
use crate::lexer::{constants::Pos, token::token::TokenKind};
use std::collections::VecDeque;

/// The maximum amount of steps to store.
pub(crate) const MAX_TRACE_LEN: usize = 1024;

/// An Error generated by the Lexer
pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
    /// The last MAX_TRACE_LEN steps done by the Lexer.
    pub(crate) traceback: VecDeque<Step>,
    /// Max number of steps to display.
    pub(crate) depth: usize
}

pub(crate) enum ErrorKind {
    /// Invalid character at a given position
    InvalidSequence { chr: char, pos: Pos /* [line, colum, char] */ },
}

/// A single step done by the Lexer. (Eg. Constructing an Integer Token)
pub(crate) struct Step {
    token: TokenKind,
    /// The position where the step was done. [line, column, char]
    pos: Pos
}
