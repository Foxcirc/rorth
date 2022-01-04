
use crate::*;

pub(crate) struct Procedure<'a> {
    pub(crate) args: Vec<&'a str>,
    pub(crate) body: Bytecode<'a>,
}

impl<'a> Procedure<'a> {

    // pub(crate) fn blank() -> Self {
    //     Self {
    //         args: Vec::new(),
    //         body: Bytecode::blank(),
    //     }
    // }

    pub(crate) fn parse<I: Iterator<Item = &'a Token> + Clone>(iter: &mut I, stream: &'a Tokenstream) -> Option<(&'a str, Self)> {

        use Tokenkind::*;

        let mut name = "";
        let mut args = Vec::new();
        let body;
        let mut state = State::Norm;
        let mut idx = 0;
        let mut bodyiter = iter.clone();
        let mut start = 0;
        
        loop {
            
            let token = match iter.next() { Some(tk) => tk, None => return None, };

            //? key: "let"
            if idx == 0 && !(token.kind == KeyLet) { return None; }

            //? ident
            if idx == 1 && !(token.kind == Ident) { return None; }
            else if idx == 1 && token.kind == Ident { name = stream.rawr(token) }
            
            //? key: "proc"
            if idx == 2 && !(token.kind == KeyProc) { return None; }
            
            //? proc-args
            //? multiple args init
            if idx == 3 && token.kind == OpenNormal {
                state = State::Args;
            }
            //? single arg
            else if idx == 3 && token.kind == Ident {

                let sname = stream.rawr(token);
                args.push(sname);

                state = State::Open;
            }

            //? invalid
            else if idx == 3 && token.kind == KeyIn {
                state = State::Open;
            }
            else if idx == 3 {
                return None;
            }

            //? multiple args consume
            if state == State::Args && token.kind == Ident {

                let sname = stream.rawr(token);
                args.push(sname);

            }

            //? multiple args finish
            else if state == State::Args && token.kind == CloseNormal {
                state = State::Open;
            }

            //? multiple args invalid
            else if state == State::Args {
                return None;
            }

            //? key: "in"
            if state == State::Open && token.kind == KeyIn {
                state = State::Body;
                bodyiter.nth(idx).aborts("The iterator may not end now"); // skip to the start of the function body
                start = idx;
            }
            else if state == State::Open {
                return None;
            }

            if state == State::Body && token.kind == KeyEnd {
                // idx is now the "end" of the iterator
                let mut ranged = bodyiter.clone().enumerate().filter(|a| a.0 < (idx - start - 1)).map(|a| a.1);
                body = Bytecode::parse(&mut ranged, &stream);
                
                return Some((name, Self { args, body }));

            }
            else if state == State::Body {
                // do nothing
            }

            idx += 1;

        };

    }

}

#[derive(PartialEq)]
enum State {
    Norm,
    Args,
    Open,
    Body,
}
