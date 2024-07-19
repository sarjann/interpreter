use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token};
// use std::fmt::{Debug, Display, Formatter};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();
        return parser;
    }

    pub fn parse(&mut self) -> statements::ProgramStatement {
        let mut bs = statements::ProgramStatement { body: Vec::new() };

        // self.next_token();
        while !matches!(self.current_token, Token::Eof) {
            let statement = self.parse_statement();

            if statement.is_some() {
                bs.body.push(statement.unwrap());
            }
            self.next_token();
        }

        // Printing output
        println!("Output:");
        for statement in bs.body.iter() {
            println!("{}", statement.literal());
        }

        if self.errors.len() > 0 {
            println!("errors");
            for err in self.errors.iter() {
                println!("{}", err);
            }
        }

        return bs;
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let statement: Option<Box<dyn Statement>> = match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => None,
        };
        return statement;
    }

    fn peek_error(&mut self, token: Token) {
        self.errors.push(format!(
            "Expected {} but got {} instead",
            token, self.peek_token
        ));
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
            self.next_token();
            return true;
        } else {
            println!("err");
            self.peek_error(token);
            return false;
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expect_peek(Token::Ident(String::new())) {
            return None;
        };

        let ident_token = self.current_token.clone();

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        let value = self.peek_token.clone();
        let value_expression = expressions::ExpressionNode {
            value: value.literal(),
        };

        let identifier = expressions::IdentifierExpression { token: ident_token };

        let statement = statements::LetStatement {
            token: Token::Let,
            name: identifier,
            value: Box::new(value_expression),
        };

        // TODO (recursive stuff)

        if self.current_token != Token::Semicolon {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let return_value = Box::new(expressions::ExpressionNode {
            value: "test".to_string(),
        });
        self.next_token();
        // TODO filling expression (return_value)

        let statement = statements::ReturnStatement {
            token: Token::Return,
            return_value,
        };

        if self.current_token != Token::Semicolon {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Statement>> {
        return None;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        return ();
    }
}
