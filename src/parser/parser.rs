
use std::collections::HashMap;
use crate::*;

pub(crate) type TypeID = u32;
pub(crate) type Procedures = HashMap<String /* todo use &str + one big string to be more efficient (maybe?) */, Procedure>;
pub(crate) type Constants = HashMap<TypeID, Constant>;
pub(crate) type Structures = Vec<Structure>;
pub(crate) type StructureNames = HashMap<String /* did you know this ^^ counts for the `Value`s too? (and for `Structure`s :O) */, TypeID>;
pub(crate) type Environment = (Constants, Procedures, Structures);

pub(crate) struct Parser {
}

impl Parser {
    
    pub(crate) fn new() -> Self {
        
        Self {}

    }

    pub(crate) fn build(self, tokens: &Tokenstream) -> Result<(Bytecode, Environment), ()> {

        let program = self.parse(tokens);
        Ok(program)

    }

    pub(crate) fn parse(&self, tokens: &Tokenstream) -> (Bytecode, Environment) {
        
        const EOF: &str = "Unexpected end of file.";

        use Tokenkind::*;
        use Instruction as Ins;
        
        macro_rules! initps { ($($name:ident($align:literal, $size:literal)),*) => { {
            let mut items = Vec::new(); let mut names = StructureNames::new(); let mut count = 0;
            $(items.push(Structure::primitive($size, $align)); names.insert(stringify!($name).to_owned(), count); count += 1; )*
            (items, names)
        } }; }

        let mut bcode = Bytecode::new(0);
        let consts = Constants::new();
        let procs = Procedures::new();
        let (structs, names) = initps![int(8, 8), bool(2, 2), ref(8, 8)];

        let mut iter = tokens.iter();

        let iskwd = |token, kwd| tokens.iskwd(token, kwd);
        let read  = |token| tokens.read(token);
        let rawr  = |token| tokens.rawr(token);
        let eof   = || {
            let diag = Diag::new();
            diag.level(Level::Error);
            diag.say(EOF);
            diag
        };

        loop {

            let token = match iter.next() { Some(next) => next, None => break, };
            match token.kind {

                Ident if iskwd(token, "let") => {

                    let token = iter.next().abortsby(&eof());
                    if token.kind != Ident { Diag::new().level(Level::Error).say("`let` must be followed by a name").note("eg: `let hello const \"Hello world\"`").abort(); }
                    
                    let token = iter.next().abortsby(&eof());
                    
                    match token.kind {

                        Ident if iskwd(token, "proc") => {

                            fatal!("Found a procedure YES BABY");

                        }

                        _ => Diag::error(&format!("Expected `proc` but got `{:?}`", rawr(token))),

                    }

                },

                Newline => (),

                other => fatal!("Token `{:?}` is not implemented yet.", other),

            }

        }

        return (bcode, (consts, procs, structs))

    }

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
                let integer: u64 = slice.parse().aborts("The integer token is not valid.");
                Readout::Integer(integer)
            },
            
            Kind::Float => {
                let float: f64 = slice.parse().aborts("The integer token is not valid.");
                Readout::Float(float)
            },
            
            Kind::Literal => Readout::Literal(slice),
            Kind::Ident => Readout::Ident(slice),

            Kind::Comment => Readout::Invalid,
            Kind::Note => Readout::Invalid,

            _ => Readout::Invalid,
        }

    }

    #[inline]
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
    pub(crate) fn int(self) -> u64 {
        if let Self::Integer(v) = self { v } else { Diag::fatal(&format!("Expected the `Readout::Integer` but got {:?}", self)) }
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
        if let Self::Ident(v) = self { v } else { Diag::fatal(&format!("Expected the `Readout::Ident` but got {:?}", self)) }
    }

}
