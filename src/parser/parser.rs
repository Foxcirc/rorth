
use std::collections::HashMap;
use crate::*;

pub(crate) type TypeID = u32;
pub(crate) type Procedures<'a> = HashMap<&'a str, Procedure<'a>>;
pub(crate) type Constants<'a> = HashMap<&'a str, Constant>;
pub(crate) type Structures<'a> = HashMap<&'a str, Structure>;

pub(crate) struct Parser {
}

impl Parser {
    
    pub(crate) fn new() -> Self {
        
        Self {}

    }

    pub(crate) fn build<'a>(self, tokens: &'a Tokenstream) -> Result<Environment<'a>, ()> {

        let program = self.parse(tokens);
        Ok(program)

    }

    pub(crate) fn parse<'a>(&self, tokens: &'a Tokenstream) -> Environment<'a> {
        
        use Tokenkind::*;
        
    
        macro_rules! initps { ($($name:ident($align:literal, $size:literal)),*) => { {
            let mut items = HashMap::new();
            $(items.insert(stringify!($name), Structure::primitive($size, $align));)*
            items
        } }; }

        let consts = Constants::new();
        let mut procs = Procedures::new();
        let structs = initps![int(8, 8), bool(2, 2), ref(8, 8)];
        

        let mut iter = tokens.iter().filter(|tk| tk.kind != Newline);

        loop {

            // todo make iter peekable and peek for efficiency

            if let Some((name, proc)) = Procedure::parse(&mut iter, &tokens) {
                procs.insert(name, proc);
            }
            else {
                Diag::error("Could not match to any sytnax pattern.");
            };

            if iter.size_hint().1 == Some(0) { break };

        }

        return Environment { consts, procs, structs, }

    }

}

pub(crate) struct Environment<'a> {
    pub(crate) consts: Constants<'a>,
    pub(crate) procs: Procedures<'a>,
    pub(crate) structs: Structures<'a>,
}

impl Environment<'_> {
    
    pub(crate) fn blank() -> Self {
        Self {
            consts: Constants::new(),
            procs: Procedures::new(),
            structs: Structures::new(),
        }
    }

}
