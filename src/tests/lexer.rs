
use crate::lexer::*;

// why is this working?

#[test]
fn vis() {
    
    let code = r#"hello ´world"#;
    // let code = r#"let main proc - int in 0 end"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);

}

#[test]
fn real() {
    
    let text = include_str!("../../stuff/struct.rh");

    let lexer = Lexer::new(&text);
    let tokens = lexer.build().unwrap();

    println!("{}", tokens.tokens.len() * std::mem::size_of::<Token>());
    
    // (no sep.)   3568 with 64-bit enum
    // (no sep.)   1784 with 8-bit  enum (gets 8 bit padding inside the "Token{}" struct)
    // (with sep.) 3032 with 8-bit  enum (gets padding again)

}

#[test]
fn comments() {

    const EXPECTED: &str = "";

    let code = r#"69`ccc`"lll"`ccc"xxx"`"lll`xxx`"69"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);
    assert!(&format!("{}", &tokens) .contains(EXPECTED));
    
}

#[test]
fn identifiers() {

    const EXPECTED: &str = r#"Tokenstream[`__hello-world__` `+` `abc69` `+` `420` `..` `a1b2-`]"#;

    let code = r#"__hello-world__ + abc69 + 420 .. a1b2-"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);
    assert!(&tokens.to_string() == EXPECTED);
}
 
#[test]
fn literals() {

    const EXPECTED: &str = r#"Tokenstream[`Hello world!` `+` `69` `+` `\n` `You are my life`]"#;
    
    let code = "\"Hello world!\" + 69 + \n \"You are my life\"";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);
    assert!(tokens.to_string() == EXPECTED);
}

#[test]
fn numbers() {

    const EXPECTED: &str = r#"Tokenstream[`1` `22` `333` `4444` `55555` `666666` `7777777` `88888888` `999999999` `1.` `2.2` `3.33` `44.44` `555.55` `666.666` `1_` `2_2` `3_3_3` `4____444` `5__5__55` `666666_______`]"#;
    
    let code = "1 22 333 4444 55555 666666 7777777 88888888 999999999 1. 2.2 3.33 44.44 555.55 666.666 1_ 2_2 3_3_3 4____444 5__5__55 666666_______";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);
    assert!(tokens.to_string() == EXPECTED);

}

#[test]
fn doubles() {

    const EXPECTED: &str = r#"Tokenstream[`..` `..` `==` `!=` `<=` `>=` `..` `.` `:` `!` `!=` `!=` `~` `1` `==` `2` `!=` `.`]"#;

    let code = "....==!=<=>=...:!!=!=~1==2!=.";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", &tokens);
    assert!(&format!("{}", &tokens) == EXPECTED);

}

#[test]
fn singles() {
    
    const EXPECTED: &str = r#"Tokenstream[`\n` `\n` `1` `22` `333` `444` `5555` `666666` `7777777` `88888888` `999999999` `\n` `1` `+` `22` `+` `333` `+` `+` `+` `-` `-` `-` `*` `*` `*` `/` `/` `/` `+` `+` `-` `-` `*` `*` `/` `/` `\n` `1` `+` `2` `-` `3` `*` `4` `/` `5` `11` `-` `22` `\n` `\n`]"#;

    let code = "
    
        1 22 333 444 5555 666666 7777777 88888888 999999999
        1 + 22 + 333 +++ --- *** /// ++--**//
        1+2-3*4/5 11-22
    
    ";
    
    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{}", tokens);
    assert!(tokens.to_string() == EXPECTED);
    
}
