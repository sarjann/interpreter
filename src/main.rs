use lexer::Lexer;
// use ast::AST;
use std::fs;

fn main() {
    let source_file = "file.lang";
    let input_string = fs::read_to_string(source_file).expect(
        "Issue loading\
        input string",
    );
    let mut lexer = Lexer::new(input_string);
    let tokens = lexer.run();
    Lexer::print_tokens(&tokens);
}
