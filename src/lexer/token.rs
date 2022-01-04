
use enumflags2::bitflags;
use std::fmt::{self as format, Display, Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::*;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) kind: Tokenkind,
    pub(crate) pos: u32,
    pub(crate) length: u16,
}

impl Token {
    
    pub(crate) fn new(kind: Tokenkind, pos: u32, length: u16) -> Self {
        Self { pos, kind, length }
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
    pub(crate) tokens: Vec<Token>,
    pub(crate) text: &'a str,
}

impl<'a> Tokenstream<'a> {

    pub(crate) fn new(size: usize, text: &'a str) -> Self {
        Self { tokens: Vec::with_capacity(size), text }
    }

    pub(crate) fn rawr(&self, tk: &Token) -> &'a str { // rawr <3
        &self.text[(tk.pos as usize)..=((tk.pos + tk.length as u32) as usize - 1)]
    }

    pub(crate) fn rint(&self, tk: &Token) -> u64 {
        self.rawr(tk).parse().aborts("Invalid integer token.")
    }

}

impl Deref for Tokenstream<'_> {
    type Target = Vec<Token>;
    fn deref(&self) -> &Self::Target { &self.tokens }
}

impl DerefMut for Tokenstream<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.tokens }
}

impl Display for Tokenstream<'_> { // todo make it beatiful :sparkle:
    
    fn fmt(&self, fmt: &mut Formatter) -> format::Result {

        fmt.write_str("Tokenstream(")?;
        
        for (index, token) in self.tokens.iter().enumerate() {
            token.kind.fmt(fmt)?;

            if token.length != 1 {
                fmt.write_str("[")?;
                Display::fmt(&token.pos, fmt)?;
                fmt.write_str("|")?;
                Display::fmt(&token.length, fmt)?;
                fmt.write_str("]")?;
            };

            if (index + 1) != self.tokens.len() { fmt.write_str(", ")?; };
        }

        fmt.write_str(")")?;

        Ok(())

    }

}
