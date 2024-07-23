use ast::expressions::*;
use ast::statements::*;
use lexer::{Lexer, Token};
use parser::Parser;
use std::fs;

#[test]
fn parse_let() {
    let v: Vec<u8> = fs::read("tests/test_files/let.lang").expect("Couldn't open file");
    let lex = Lexer::new(v);
    let mut parser = Parser::new(lex);
    let parsed_statement = parser.parse();

    let expected = ProgramStatement {
        body: vec![Box::new(LetStatement {
            token: Token::Let,
            name: IdentifierExpression {
                token: Token::Ident("x".to_string()),
            },
            value: Box::new(ExpressionNode {
                value: "10".to_string(),
            }),
        })],
    };
    assert_eq!(parsed_statement, expected);
    let body = parsed_statement.body;
    for value in body.iter() {
        println!("{:#?}", value);
    }
}
