
use crate::lexer::*;

#[derive(Clone)]
pub(crate) enum Node {
    Key(&'static str),
    Value(&'static str, Tokenkind),
    Complex(Vec<Node>),
    Multiple(Vec<Node>),
    Optional(Box<Node>),
    Repeat(Box<Node>),
    Labeled(&'static str, Box<Node>),
}

pub(crate) enum Traversed {
    Procedure,
    Invalid,
}
