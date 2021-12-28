
#[cfg(test)]
mod tests;
mod lexer;
mod parser;
mod sim;

#[allow(dead_code)]
mod error;

use std::fs::File;
use std::io::Read;
use error::*;

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

    let tokens = lexer::Lexer::new(&code).build().unwrap();
    let (code, env) = parser::Parser::new(tokens).build().unwrap();

    Diag::info("starting in simulation mode");

    let mut sim = sim::Simulator::new();
    sim.setup(env);
    sim.run(code);

    for value in sim.stack.iter() {
        println!("{:?}", value.view());
    };

}

fn getcode() -> (String, String) {
    let path = std::env::args().collect::<Vec<String>>().get(1).map(|v| v.clone()).expect("The first argument must be the file path.");
    let mut file = File::open(&path).expect("Cannot open file.");
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    (path, code)
}
