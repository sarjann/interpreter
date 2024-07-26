use ast::expressions::*;
use ast::statements::*;
use lexer::{Lexer, Token};
use parser::Parser;
use std::fs;

fn file_to_parser(path: &str) -> Parser {
    let v: Vec<u8> = fs::read(path).expect("Couldn't open file");
    let lex = Lexer::new(v);
    let parser = Parser::new(lex);
    return parser;
}

fn string_to_parser(string: &str) -> Parser {
    let v: Vec<u8> = string.bytes().collect();
    let lex = Lexer::new(v);
    let parser = Parser::new(lex);
    return parser;
}

#[test]
fn parse_let() {
    let mut parser = file_to_parser("tests/test_files/let.lang");
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
}

#[test]
fn parse_return() {
    let mut parser = file_to_parser("tests/test_files/return.lang");
    let parsed_statement = parser.parse();

    let expected = ProgramStatement {
        body: vec![Box::new(ReturnStatement {
            token: Token::Return,
            return_value: Box::new(ExpressionNode {
                value: "10".to_string(),
            }),
        })],
    };
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_identifier_token() {
    let mut parser = string_to_parser("test_identifier;");
    let parsed_statement = parser.parse();

    let expected = ProgramStatement {
        body: vec![Box::new(ExpressionStatement {
            token: Token::Ident("test_identifier".to_string()),
            expression: Box::new(ExpressionNode {
                value: "test_identifier".to_string(),
            }),
        })],
    };

    if parser.errors.len() > 0 {
        panic!("Failed")
    }
    assert_eq!(parsed_statement, expected);
}
