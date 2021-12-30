
use std::collections::HashMap;
use crate::*;

pub(crate) type TypeID = u32;
pub(crate) type Procedures = HashMap<String /* todo use &str + one big string to be more efficient (maybe?) */, Procedure>;
pub(crate) type Constants = HashMap<TypeID, Constant>;
pub(crate) type Structures = Vec<Structure>;
pub(crate) type StructureNames = HashMap<String /* did you know this ^^ counts for the `Value`s too? (and for `Structure`s :O) */, TypeID>;
pub(crate) type Environment = (Constants, Procedures, Structures);

pub(crate) struct Parser {
    grammar: Option<Node>,
}

impl Parser {
    
    pub(crate) fn new() -> Self {
        
        Self { grammar: None }

    }

    pub(crate) fn build(mut self, tokens: Tokenstream) -> Result<(Bytecode, Environment), ()> {

        self.setup(Self::defgrm());
        let program = self.parse(tokens);
        Ok(program)

    }

    fn defgrm() -> Node {

        use Tokenkind::*;

        /* 
        gmr![
            proc: ["let", ident, "proc", ?(ident || ["(", ident*, ")"]) "in" &expr* ]
        ]
        */

        // let expr = Node::Labeled("expr", Box::new(
        //     Node::Value("expr", Ident),
        // ));

        let proc = Node::Labeled("proc", Box::new(
            Node::Complex(vec![
                Node::Key("let"),
                Node::Value("name", Ident),
                Node::Key("proc"),
                /* arguments */
                Node::Optional(Box::new(Node::Multiple(vec![
                    Node::Value("args", Ident),
                    Node::Complex(vec![
                        Node::Key("("),
                        Node::Repeat(Box::new(Node::Value("args", Ident))),
                        Node::Key(")"),
                    ])
                ]))),
                /* returns */
                Node::Optional(Box::new(Node::Complex(vec![
                    Node::Key("-"),
                    Node::Multiple(vec![
                    Node::Value("rets", Ident),
                    Node::Complex(vec![
                        Node::Key("("),
                        Node::Repeat(Box::new(Node::Value("rets", Ident))),
                        Node::Key(")"),
                    ])
                ])]))),
                /* body */
                Node::Key("in"),
                Node::Repeat(Box::new(Node::Value("expr", Ident))),
                Node::Key("end"),
            ]))
        );

        let grammar = Node::Multiple(vec![
            proc,
        ]);

        grammar
    }

    pub(crate) fn setup(&mut self, grammar: Node) {
        self.grammar = Some(grammar);
    }

    pub(crate) fn parse(&self, tokens: Tokenstream) -> (Bytecode, Environment) {
        
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
        let (structs, _names) = initps![int(8, 8), bool(2, 2), ref(8, 8)];

        // let isvalid = Self::validgrm(self.grammar.as_ref().aborts("The grammar must be set before parsing."));
        // if !isvalid {
        //     Diag::fatal("The grammar is not valid.");
        // }

        todo!();

        // return (bcode, (consts, procs, structs))

    }

    /* 
        fn validgrm(grm: &Node) -> bool {

            let valids = Self::validate(grm, ([false], false));
            
            valids.0.iter().all(|v| v == &true) // && valids.1

        }

        fn validate(grm: &Node, mut found: ([bool; 1], bool)) -> ([bool; 1], bool) {

            use Node::*;

            // found: [proc]

            match grm {

                Complex(nodes) => for node in nodes { found = Self::validate(node, found) },
                Multiple(nodes) => for node in nodes { found = Self::validate(node, found) },
                Optional(node) => found = Self::validate(node, found),
                Repeat(node) => found = Self::validate(node, found),

                Labeled("proc", node) => {
                    found.0[0] = true;
                    found = Self::validate(node, found);
                }

                Labeled(_, node) => {
                    found.1 = false;
                    found = Self::validate(node, found);
                }

                _ => (),

            }

            found
        }
    */
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
