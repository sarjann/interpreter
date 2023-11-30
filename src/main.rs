use lexer::{Lexer, Token};
// use ast::AST;
// use std::fs;

fn main() {
    // let source_file = "file.lang";
    // let input_string = fs::read_to_string(source_file).expect(
    //     "Issue loading\
    //     input string",
    // );
    let input_string = "let x = 5 + 5;".to_string();
    let input_bytes = input_string.into_bytes();


    let mut lexer = Lexer::new(input_bytes);
    loop {
        let token = lexer.next_token();
        println!("{}", token);
        match token {
            Token::Eof => break,
            _ => (),
        }
    }
}
