use ast::expressions;
use ast::statements;
use lexer::{Lexer, Token, TokenType};
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
    let mut parser = string_to_parser("let x = 10;");
    let parsed_statement = parser.parse();

    let expected = statements::ProgramStatement {
        body: vec![Box::new(statements::LetStatement {
            token: Token::new(TokenType::Let, None),
            name: expressions::Identifier {
                token: Token::new(TokenType::Ident, Some(String::new())),
                value: String::new(),
            },
            value: None,
        })],
    };
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_return() {
    let mut parser = string_to_parser("return 10;");
    let parsed_statement = parser.parse();

    let expected = statements::ProgramStatement {
        body: vec![Box::new(statements::ReturnStatement {
            token: Token::new(TokenType::Return, None),
            return_value: None,
        })],
    };
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_identifier_token() {
    let mut parser = string_to_parser("test_identifier;");
    let parsed_statement = parser.parse();

    let expected = statements::ProgramStatement {
        body: vec![Box::new(statements::ExpressionStatement {
            token: Token::new(TokenType::Ident, Some("test_identifier".to_string())),
            expression: None,
        })],
    };

    if parser.errors.len() > 0 {
        panic!("Failed")
    }
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_prefix_expression() {
    let mut parser = string_to_parser("!10;");
    let parsed_statement = parser.parse();

    let expected = statements::ProgramStatement {
        body: vec![Box::new(statements::ExpressionStatement {
            token: Token::new(TokenType::Bang, None),
            expression: Some(Box::new(expressions::Prefix {
                token: Token::new(TokenType::Bang, None),
                operator: "!".to_string(),
                right: Some(Box::new(expressions::IntegerLiteral {
                    token: Token::new(TokenType::Int, Some("10".to_string())),
                    value: 10,
                })),
            })),
        })],
    };

    if parser.errors.len() > 0 {
        panic!("Failed")
    }
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_integer_literal_expression() {
    let mut parser = string_to_parser("10;");
    let parsed_statement = parser.parse();

    let expected = statements::ProgramStatement {
        body: vec![Box::new(statements::ExpressionStatement {
            token: Token::new(TokenType::Int, Some("10".to_string())),
            expression: Some(Box::new(expressions::IntegerLiteral {
                token: Token::new(TokenType::Int, Some("10".to_string())),
                value: 10,
            })),
        })],
    };
    if parser.errors.len() > 0 {
        panic!("Failed")
    }
    assert_eq!(parsed_statement, expected);
}

#[test]
fn parse_infix_expression() {
    struct TestInput {
        input: String,
        left_value: i64,
        operator: String,
        right_value: i64,
        token: Token,
    }
    let test_inputs: Vec<TestInput> = vec![
        TestInput {
            input: "5 + 5;".to_string(),
            left_value: 5,
            operator: "+".to_string(),
            right_value: 5,
            token: Token::new(TokenType::Plus, None),
        },
        TestInput {
            input: "5 - 3;".to_string(),
            left_value: 5,
            operator: "-".to_string(),
            right_value: 3,
            token: Token::new(TokenType::Minus, None),
        },
        TestInput {
            input: "2 * 5;".to_string(),
            left_value: 2,
            operator: "*".to_string(),
            right_value: 5,
            token: Token::new(TokenType::Asterisk, None),
        },
        TestInput {
            input: "5 / 5;".to_string(),
            left_value: 5,
            operator: "/".to_string(),
            right_value: 5,
            token: Token::new(TokenType::Slash, None),
        },
        TestInput {
            input: "5 > 5;".to_string(),
            left_value: 5,
            operator: ">".to_string(),
            right_value: 5,
            token: Token::new(TokenType::RT, None),
        },
        TestInput {
            input: "5 < 5;".to_string(),
            left_value: 5,
            operator: "<".to_string(),
            right_value: 5,
            token: Token::new(TokenType::LT, None),
        },
        TestInput {
            input: "5 == 5;".to_string(),
            left_value: 5,
            operator: "==".to_string(),
            right_value: 5,
            token: Token::new(TokenType::Eq, None),
        },
        TestInput {
            input: "5 != 5;".to_string(),
            left_value: 5,
            operator: "!=".to_string(),
            right_value: 5,
            token: Token::new(TokenType::NotEq, None),
        },
    ];

    for test_input in test_inputs.iter() {
        let mut parser = string_to_parser(&test_input.input);
        let parsed_statement = parser.parse();

        let expected = statements::ProgramStatement {
            body: vec![Box::new(statements::ExpressionStatement {
                token: Token::new(TokenType::Int, Some(test_input.left_value.to_string())),
                expression: Some(Box::new(expressions::Infix {
                    token: test_input.token.clone(),
                    operator: test_input.operator.clone(),
                    left: Some(Box::new(expressions::IntegerLiteral {
                        token: Token::new(TokenType::Int, Some(test_input.left_value.to_string())),
                        value: 5,
                    })),
                    right: Some(Box::new(expressions::IntegerLiteral {
                        token: Token::new(TokenType::Int, Some(test_input.right_value.to_string())),
                        value: 5,
                    })),
                })),
            })],
        };
        if parser.errors.len() > 0 {
            dbg!("Errors:");
            for err in parser.errors.iter() {
                dbg!("{}", err);
            }
            panic!("Failed")
        }
        assert_eq!(parsed_statement, expected);
    }
}
