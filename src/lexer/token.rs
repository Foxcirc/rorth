
use enumflags2::bitflags;
use std::fmt::{self as format, Display, Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::*;

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub(crate) kind: Tokenkind,
    pub(crate) text: &'a str,
    pub(crate) pos: usize,
}

impl<'a> Token<'a> {
    
    pub(crate) fn new(kind: Tokenkind, text: &'a str, pos: usize) -> Self {
        Self { kind, text, pos }
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }

    pub(crate) fn asi(&self) -> u64 {
        if self.kind != Tokenkind::Integer { fatal!("expected the token to be an integer") };
        self.text.parse().aborts("invalid integer token")
    }

    pub(crate) fn asf(&self) -> f64 {
        if self.kind != Tokenkind::Float { fatal!("expected the token to be a float") };
        self.text.parse().aborts("invalid float token")
    }

}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Tokenkind {

    Seperator, // spaces, tabs, \r, etc.
    Newline, // newline
    Plus,
    Minus, 
    Star,
    Slash,
    Dot,
    Colon,
    Comma,
    Equals,
    Bang, // write to memory
    Ampersand,
    Percent,
    Tilde,
    Hashtag,
    Questionmark, // read from memory
    Pipe,
    
    OpenSharp,
    CloseSharp,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenNormal,
    CloseNormal,
    
    DoubleDot,
    EqualsEquals,
    BangEquals,
    OpenSharpEquals,
    CloseSharpEquals,
    MinusEquals, // sub. intrinsic
    PlusEquals, // add. intr.
    StarEquals, // mul. intr.
    SlashEquals, // div. intr.
    PercentEquals, // mod. intr.

    Integer,
    Float,
    Ident,
    KeyLet,
    KeyProc,
    KeyIn,
    KeyEnd,
    Literal,
    Comment,
    Note,
}

#[derive(Debug)]
pub(crate) struct Tokenstream<'a> {
    pub(crate) tokens: Vec<Token<'a>>,
    pub(crate) text: &'a str,
}

impl<'a> Tokenstream<'a> {

    pub(crate) fn new(size: usize, text: &'a str) -> Self {
        Self { tokens: Vec::with_capacity(size), text }
    }

}

impl<'a> Deref for Tokenstream<'a> {
    type Target = Vec<Token<'a>>;
    fn deref(&self) -> &Self::Target { &self.tokens }
}

impl DerefMut for Tokenstream<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.tokens }
}

impl Display for Tokenstream<'_> { // todo make it beatiful :sparkle:
    
    fn fmt(&self, fmt: &mut Formatter) -> format::Result {

        fmt.write_str("Tokenstream[")?;
        
        for (index, token) in self.tokens.iter().enumerate() {

            match token.kind {

                Tokenkind::Comment | Tokenkind::Note => {
                    fmt.write_str("`{comment}`")?;
                },

                _ => {
                    fmt.write_str("`")?;
                    fmt.write_str(&token.text.escape_debug().to_string())?;
                    fmt.write_str("`")?;
                },
            }

            if (index + 1) != self.tokens.len() { fmt.write_str(" ")?; };
        }

        fmt.write_str("]")?;

        Ok(())

    }

}
