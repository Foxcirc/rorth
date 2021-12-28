
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::parser::*;

#[derive(Debug)]
pub(crate) enum Instruction {
    
    Push(Value),

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

pub(crate) struct Bytecode {
    pub(crate) instrs: Vec<Instruction>,
}

impl Bytecode {
    
    pub(crate) fn new(size: usize) -> Self {
        Self { instrs: Vec::with_capacity(size) }
    }

}

impl Deref for Bytecode { type Target = Vec<Instruction>; fn deref(&self) -> &Self::Target { &self.instrs } }
impl DerefMut for Bytecode { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.instrs } }

impl Display for Bytecode {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        
        fmt.write_str("Bytecode:\n")?;

        for item in self.instrs.iter() {
            fmt.write_str("  ")?;
            Debug::fmt(item, fmt)?;
            fmt.write_str("\n")?;
        };

        Ok(())

    }
}
