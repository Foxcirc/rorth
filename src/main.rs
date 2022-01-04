
#[cfg(test)]
mod tests;
mod lexer;
mod parser;
mod sim;

#[allow(dead_code)]
mod error;

use std::fs::File;
use std::io::Read;

pub(crate) use error::*;
pub(crate) use lexer::*;
pub(crate) use parser::*;
pub(crate) use sim::*;

fn main() {

    let (path, code) = getcode();
    
    Diag::info(&format!("compiling {}", &path));

    // Diag::new()
    //     .level(Level::Error)
    //     .say("use of unstable feature `enums`")
    //     .code("let Tokenkind enum {")
    //     .file("lexer/token.rs")
    //     .pos(lexer::Location::new(0, 16, 16))
    //     .hint("enable the feature using `#{unstable-feature: enums}`")
    //     .hint("enums aren't stable yet, please consider using `std:Enum` for now")
    // .emit();

    let tokens = lexer::Lexer::new(&code).build().aborts("Could not generate tokens.");
    let mut env = parser::Parser::new().build(&tokens).aborts("Could not generate bytecode.");
    
    if !env.procs.contains_key("main") {
        Diag::error("missing `main` procedure");
    }

    let main = env.procs.remove("main").aborts("must have a `main` procedure");

    Diag::info("starting in simulation mode");

    let mut sim = sim::Simulator::new();

    sim.setup(env);
    sim.run(main);

    for value in sim.stack.iter() {
        println!("{:?}", value.view());
    };

}

fn getcode() -> (String, String) {
    let path = std::env::args().collect::<Vec<String>>().get(1).map(|v| v.clone()).aborts("The first argument must be the file path.");
    let mut file = File::open(&path).aborts("Could not open source file.");
    let mut code = String::new();
    file.read_to_string(&mut code).aborts("Could not read source file.");
    (path, code)
}
