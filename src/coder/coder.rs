
use std::collections::HashMap;
use crate::lexer::*;
use crate::coder::*;

pub(crate) type TypeID = u32;
pub(crate) type Procedures = HashMap<String /* todo use &str + one big string to be more efficient (maybe?) */, Procedure>;
pub(crate) type Constants = HashMap<TypeID, Constant>;
pub(crate) type Structures = Vec<Structure>;
pub(crate) type StructureNames = HashMap<String /* did you know this ^^ counts for the `Value`s too? (and for `Structure`s :O) */, TypeID>;
pub(crate) type Program = (Bytecode, Constants, Procedures, Structures);

pub(crate) struct Coder<'a> {
    tokens: Tokenstream<'a>,
}

impl<'a> Coder<'a> {
    
    pub(crate) fn new(tokens: Tokenstream<'a>) -> Self {
        
        Self { tokens }

    }

    pub(crate) fn build(self) -> Result<Program, ()> {

        let program = self.parse();

        // self.check(&program);

        Ok(program)

    }

    fn parse(&self) -> Program {
        
        use Tokenkind::*;
        use Instruction as Ins;
        
        macro_rules! initps { ($($name:ident($align:literal, $size:literal)),*) => { {
            let mut items = Vec::new(); let mut names = StructureNames::new(); let mut count = 0;
            $(items.push(Structure::primitive($align, $size)); names.insert(stringify!($name).to_owned(), count); count += 1; )*
            (items, names)
        } }; }

        let tokens = &self.tokens;
        let mut bcode = Bytecode::new(0);
        let consts = Constants::new();
        let procs = Procedures::new();
        let (structs, _names) = initps![int(8, 8), bool(2, 2), ref(8, 8)];

        for token in tokens.iter() {

            match token.kind {

                Newline => (),

                PlusEquals  => bcode.push(Ins::Add),
                MinusEquals => bcode.push(Ins::Subtract),
                StarEquals  => bcode.push(Ins::Multiply),
                SlashEquals => bcode.push(Ins::Divide),
                // todo add TokenKind::Keyword to simlify checks for "naming things after intrinsics": Eg. "let drop bind int"
                Ident if tokens.iskwd(token, "dup")  => { bcode.push(Ins::Dup)      },
                Ident if tokens.iskwd(token, "swap") => { bcode.push(Ins::Swap)     },
                Ident if tokens.iskwd(token, "over") => { bcode.push(Ins::Over)     },
                Ident if tokens.iskwd(token, "drop") => { bcode.push(Ins::Drop)     },
                Ident if tokens.iskwd(token, "rotl") => { bcode.push(Ins::RotLeft)  },
                Ident if tokens.iskwd(token, "rotr") => { bcode.push(Ins::RotRight) },

                Integer => {
                    let value = tokens.read(&token).integer();
                    bcode.push(Ins::Push(Value::make(0, &structs[0], value.to_ne_bytes())));
                },

                other => todo!("This token is not implemented yet: {:?}", other),

            }

        };

        (bcode, consts, procs, structs)

    }

    // fn check(&self, program: &Program) {

    //     // use Instruction as Ins;

    //     // let mut stack: Vec<TypeID> = Vec::new();

    //     for instr in program.0.iter() {

    //         match instr {
    //             other => todo!("This instruction is not implemented for meta evaluation: {:?}", other),
    //         }

    //     }

    // }

}

trait ReadToken {
    fn read(&self, token: &Token) -> Readout;
    fn iskwd(&self, token: &Token, name: &'static str) -> bool;
}

impl<'a> ReadToken for Tokenstream<'a> {
    #[inline]
    fn read(&self, token: &Token) -> Readout {

        use Tokenkind as Kind;

        let slice = &self.text[(token.pos as usize)..=((token.pos + token.length as u32) as usize - 1)];

        match token.kind {

            Kind::Integer => {
                let integer: u64 = slice.parse().expect("The integer token is not valid.");
                Readout::Integer(integer)
            },
            
            Kind::Float => {
                let float: f64 = slice.parse().expect("The integer token is not valid.");
                Readout::Float(float)
            },
            
            Kind::Literal => Readout::Literal(slice),
            Kind::Ident => Readout::Ident(slice),

            Kind::Comment => Readout::Invalid,
            Kind::Note => Readout::Invalid,

            _ => Readout::Invalid,
        }

    }

    fn iskwd(&self, token: &Token, name: &'static str) -> bool {
        self.read(token).ident() == name
    }
}

#[derive(Debug)]
enum Readout<'a> {
    Integer(u64),
    Float(f64),
    Literal(&'a str),
    Ident(&'a str),
    Invalid,
}

impl<'a> Readout<'a> {
    
    #[inline(always)]
    pub(crate) fn integer(self) -> u64 {
        if let Self::Integer(v) = self { v } else { unreachable!("Expected the `Readout::Integer` but got {:?}", self) }
    }
    
    // #[inline(always)]
    // pub(crate) fn float(self) -> f64 {
    //     if let Self::Float(v) = self { v } else { unreachable!("Expected the `Readout::Float` but got {:?}", self) }
    // }
    
    // #[inline(always)]
    // pub(crate) fn literal(self) -> &'a str {
    //     if let Self::Literal(v) = self { v } else { unreachable!("Expected the `Readout::Literal` but got {:?}", self) }
    // }
    
    #[inline(always)]
    pub(crate) fn ident(self) -> &'a str {
        if let Self::Ident(v) = self { v } else { unreachable!("Expected the `Readout::Ident` but got {:?}", self) }
    }

}
