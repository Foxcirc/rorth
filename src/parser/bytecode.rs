
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::*;

#[derive(Debug)]
pub(crate) enum Instruction<'a> {
    
    Push(Value<'a>),

    // Math ops
    Add,
    Subtract,
    Multiply,
    Divide,
    //// Modulus,

    Dup,
    Swap,
    Over,
    Drop,
    RotLeft,
    RotRight,

}

pub(crate) struct Bytecode<'a> {
    pub(crate) ops: Vec<Instruction<'a>>,
}

impl<'a> Bytecode<'a> {

    pub(crate) fn blank() -> Self {
        Self { ops: Vec::new() }
    }

    pub(crate) fn parse<I: Iterator<Item = &'a Token>>(iter: &mut I, stream: &Tokenstream<'a>) -> Bytecode<'a> {
        
        use Tokenkind::*;
        use Instruction as Ins;

        let mut bcode = Self::blank();

        for token in iter {

            match token.kind {

                PlusEquals  => bcode.push(Ins::Add),
                MinusEquals => bcode.push(Ins::Subtract),
                StarEquals  => bcode.push(Ins::Multiply),
                SlashEquals => bcode.push(Ins::Divide),

                Ident if stream.rawr(token) == "dup" => bcode.push(Ins::Dup),
                Ident if stream.rawr(token) == "swap" => bcode.push(Ins::Swap),
                Ident if stream.rawr(token) == "over" => bcode.push(Ins::Over),
                Ident if stream.rawr(token) == "drop" => bcode.push(Ins::Drop),
                Ident if stream.rawr(token) == "rotl" => bcode.push(Ins::RotLeft),
                Ident if stream.rawr(token) == "rotr" => bcode.push(Ins::RotRight),

                Integer => bcode.push(Ins::Push(Value::make("int", &Structure::primitive(8, 8), stream.rint(token).to_ne_bytes()))),

                _ => Diag::error(&format!("Unexpected token: `{:?}`", stream.rawr(token))),

            }

        }

        bcode

    }

}

impl<'a> Deref for Bytecode<'a> { type Target = Vec<Instruction<'a>>; fn deref(&self) -> &Self::Target { &self.ops } }
impl DerefMut for Bytecode<'_> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.ops } }

impl Display for Bytecode<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        
        fmt.write_str("Bytecode:\n")?;

        for item in self.ops.iter() {
            fmt.write_str("  ")?;
            Debug::fmt(item, fmt)?;
            fmt.write_str("\n")?;
        };

        Ok(())

    }
}
