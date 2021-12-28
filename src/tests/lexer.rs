
use crate::lexer::*;

// why is this working?

#[test]
fn vis() {
    
    let code = r#"´hello   +     `swsw`"#;

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

    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: Integer, pos: 0, length: 2 }, Token { kind: Comment, pos: 3, length: 3 }, Token { kind: Literal, pos: 8, length: 3 }, Token { kind: Comment, pos: 13, length: 8 }, Token { kind: Literal, pos: 23, length: 8 }, Token { kind: Integer, pos: 32, length: 2 }], text:";

    let code = r#"69`ccc`"lll"`ccc"xxx"`"lll`xxx`"69"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", &tokens);
    assert!(&format!("{:?}", &tokens) .contains(EXPECTED));
    
}

#[test]
fn identifiers() {

    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: Ident, pos: 0, length: 15 }, Token { kind: Plus, pos: 16, length: 1 }, Token { kind: Ident, pos: 18, length: 5 }, Token { kind: Plus, pos: 24, length: 1 }, Token { kind: Integer, pos: 26, length: 3 }, Token { kind: DoubleDot, pos: 30, length: 2 }, Token { kind: Ident, pos: 33, length: 5 }], text:";

    let code = r#"__hello-world__ + abc69 + 420 .. a1b2-"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", &tokens);
    assert!(&format!("{:?}", &tokens) .contains(EXPECTED));
}

#[test]
fn literals() {

    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: Literal, pos: 1, length: 12 }, Token { kind: Plus, pos: 15, length: 1 }, Token { kind: Integer, pos: 17, length: 2 }, Token { kind: Plus, pos: 20, length: 1 }, Token { kind: Newline, pos: 22, length: 1 }, Token { kind: Literal, pos: 25, length: 15 }], text:";
    
    let code = "\"Hello world!\" + 69 + \n \"You are my life\"";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", &tokens);
    
    assert!(&format!("{:?}", &tokens) .contains(EXPECTED));
}

#[test]
fn numbers() {

    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: Integer, pos: 0, length: 1 }, Token { kind: Integer, pos: 2, length: 2 }, Token { kind: Integer, pos: 5, length: 3 }, Token { kind: Integer, pos: 9, length: 4 }, Token { kind: Integer, pos: 14, length: 5 }, Token { kind: Integer, pos: 20, length: 6 }, Token { kind: Integer, pos: 27, length: 7 }, Token { kind: Integer, pos: 35, length: 8 }, Token { kind: Integer, pos: 44, length: 9 }, Token { kind: Float, pos: 54, length: 2 }, Token { kind: Float, pos: 57, length: 3 }, Token { kind: Float, pos: 61, length: 4 }, Token { kind: Float, pos: 66, length: 5 }, Token { kind: Float, pos: 72, length: 6 }, Token { kind: Float, pos: 79, length: 7 }, Token { kind: Integer, pos: 87, length: 2 }, Token { kind: Integer, pos: 90, length: 3 }, Token { kind: Integer, pos: 94, length: 5 }, Token { kind: Integer, pos: 100, length: 8 }, Token { kind: Integer, pos: 109, length: 8 }, Token { kind: Integer, pos: 118, length: 13 }], text: \"1 22 333 4444 55555 666666 7777777 88888888 999999999 1. 2.2 3.33 44.44 555.55 666.666 1_ 2_2 3_3_3 4____444 5__5__55 666666_______\" }";
    
    let code = "1 22 333 4444 55555 666666 7777777 88888888 999999999 1. 2.2 3.33 44.44 555.55 666.666 1_ 2_2 3_3_3 4____444 5__5__55 666666_______";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", &tokens);
    assert!(&format!("{:?}", &tokens) == EXPECTED);

}

#[test]
fn doubles() {

    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: DoubleDot, pos: 0, length: 2 }, Token { kind: DoubleDot, pos: 2, length: 2 }, Token { kind: EqualsEquals, pos: 4, length: 2 }, Token { kind: BangEquals, pos: 6, length: 2 }, Token { kind: OpenSharpEquals, pos: 8, length: 2 }, Token { kind: CloseSharpEquals, pos: 10, length: 2 }, Token { kind: DoubleDot, pos: 12, length: 2 }, Token { kind: Dot, pos: 14, length: 1 }, Token { kind: Colon, pos: 15, length: 1 }, Token { kind: Bang, pos: 16, length: 1 }, Token { kind: BangEquals, pos: 17, length: 2 }, Token { kind: BangEquals, pos: 19, length: 2 }, Token { kind: Tilde, pos: 21, length: 1 }, Token { kind: Integer, pos: 22, length: 1 }, Token { kind: EqualsEquals, pos: 23, length: 2 }, Token { kind: Integer, pos: 25, length: 1 }, Token { kind: BangEquals, pos: 26, length: 2 }, Token { kind: Dot, pos: 28, length: 1 }], text: \"....==!=<=>=...:!!=!=~1==2!=.\" }";
    
    let code = "....==!=<=>=...:!!=!=~1==2!=.";

    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", &tokens);
    assert!(&format!("{:?}", &tokens) == EXPECTED);

}

#[test]
fn singles() {
    
    const EXPECTED: &str = "Tokenstream { tokens: [Token { kind: Newline, pos: 0, length: 1 }, Token { kind: Newline, pos: 5, length: 1 }, Token { kind: Integer, pos: 14, length: 1 }, Token { kind: Integer, pos: 16, length: 2 }, Token { kind: Integer, pos: 19, length: 3 }, Token { kind: Integer, pos: 23, length: 3 }, Token { kind: Integer, pos: 27, length: 4 }, Token { kind: Integer, pos: 32, length: 6 }, Token { kind: Integer, pos: 39, length: 7 }, Token { kind: Integer, pos: 47, length: 8 }, Token { kind: Integer, pos: 56, length: 9 }, Token { kind: Newline, pos: 65, length: 1 }, Token { kind: Integer, pos: 74, length: 1 }, Token { kind: Plus, pos: 76, length: 1 }, Token { kind: Integer, pos: 78, length: 2 }, Token { kind: Plus, pos: 81, length: 1 }, Token { kind: Integer, pos: 83, length: 3 }, Token { kind: Plus, pos: 87, length: 1 }, Token { kind: Plus, pos: 88, length: 1 }, Token { kind: Plus, pos: 89, length: 1 }, Token { kind: Minus, pos: 91, length: 1 }, Token { kind: Minus, pos: 92, length: 1 }, Token { kind: Minus, pos: 93, length: 1 }, Token { kind: Star, pos: 95, length: 1 }, Token { kind: Star, pos: 96, length: 1 }, Token { kind: Star, pos: 97, length: 1 }, Token { kind: Slash, pos: 99, length: 1 }, Token { kind: Slash, pos: 100, length: 1 }, Token { kind: Slash, pos: 101, length: 1 }, Token { kind: Plus, pos: 103, length: 1 }, Token { kind: Plus, pos: 104, length: 1 }, Token { kind: Minus, pos: 105, length: 1 }, Token { kind: Minus, pos: 106, length: 1 }, Token { kind: Star, pos: 107, length: 1 }, Token { kind: Star, pos: 108, length: 1 }, Token { kind: Slash, pos: 109, length: 1 }, Token { kind: Slash, pos: 110, length: 1 }, Token { kind: Newline, pos: 111, length: 1 }, Token { kind: Integer, pos: 120, length: 1 }, Token { kind: Plus, pos: 121, length: 1 }, Token { kind: Integer, pos: 122, length: 1 }, Token { kind: Minus, pos: 123, length: 1 }, Token { kind: Integer, pos: 124, length: 1 }, Token { kind: Star, pos: 125, length: 1 }, Token { kind: Integer, pos: 126, length: 1 }, Token { kind: Slash, pos: 127, length: 1 }, Token { kind: Integer, pos: 128, length: 1 }, Token { kind: Integer, pos: 130, length: 2 }, Token { kind: Minus, pos: 132, length: 1 }, Token { kind: Integer, pos: 133, length: 2 }, Token { kind: Newline, pos: 135, length: 1 }, Token { kind: Newline, pos: 140, length: 1 }], text:";

    let code = "
    
        1 22 333 444 5555 666666 7777777 88888888 999999999
        1 + 22 + 333 +++ --- *** /// ++--**//
        1+2-3*4/5 11-22
    
    ";
    
    let lexer = Lexer::new(code);
    let tokens = lexer.build().unwrap();
    
    println!("{:?}", tokens);
    assert!(&format!("{:?}", tokens) .contains(EXPECTED));
    
}

#[test]
fn tokenstream() {
    
    let mut stream = Tokenstream::new(0, "");
    stream.push(Token::new(Tokenkind::Plus, 0, 1));
    stream.push(Token::new(Tokenkind::Minus, 1, 1));
    stream.push(Token::new(Tokenkind::Star, 2, 1));
    stream.push(Token::new(Tokenkind::Slash, 3, 1));

    println!("{:?}", stream);

}
