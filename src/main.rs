use evaluator::{eval, Environment};
use lexer::Lexer;
use parser::Parser;
use std::fs;

fn main() {
    let source_file = "file.lang";
    let input_string = fs::read_to_string(source_file).expect(
        "Issue loading\
        input string",
    );
    // let input_string = "let x = 5 + 5;".to_string();
    let input_bytes = input_string.into_bytes();

    let lexer = Lexer::new(input_bytes);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    if !parser.errors.is_empty() {
        eprintln!("Parser errors:");
        for error in parser.errors.iter() {
            eprintln!(" - {}", error);
        }
        return;
    }

    let env = Environment::new();
    let evaluated = eval(&program, env);
    println!("{}", evaluated);
}
