use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token};
use std::collections::HashMap;
// use std::fmt::{Debug, Display, Formatter};

mod infix;
mod prefix;

enum Precedence {
    LOWEST = 0,
    EQUALS = 1,      // ==
    LESSGREATER = 2, // > or <
    SUM = 3,         // +
    PRODUCT = 4,     // *
    PREFIX = 5,      // -x
    CALL = 6,        // func(x)
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    prefix_parse_funcs: HashMap<Token, fn() -> Box<dyn Expression>>,
    infix_parse_funcs: HashMap<Token, fn() -> Box<dyn Expression>>,
}

impl Parser {
    fn get_prefix_parse_funcs() -> HashMap<Token, fn() -> Box<dyn Expression>> {
        let mut prefix_parse_funcs: HashMap<Token, fn() -> Box<dyn Expression>> = HashMap::new();
        prefix_parse_funcs.insert(Token::Ident, prefix::prefix_equal);
        return prefix_parse_funcs;
    }

    fn get_infix_parse_funcs() -> HashMap<Token, fn() -> Box<dyn Expression>> {
        let mut infix_parse_funcs: HashMap<Token, fn() -> Box<dyn Expression>> = HashMap::new();
        infix_parse_funcs.insert(Token::Ident, infix::infix_equal);
        return infix_parse_funcs;
    }

    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
            prefix_parse_funcs: Parser::get_prefix_parse_funcs(),
            infix_parse_funcs: Parser::get_infix_parse_funcs(),
        };

        parser.next_token();
        parser.next_token();
        return parser;
    }

    pub fn parse(&mut self) -> statements::ProgramStatement {
        let mut bs = statements::ProgramStatement { body: Vec::new() };

        while !matches!(self.current_token, Token::Eof) {
            let statement = self.parse_statement();

            if statement.is_some() {
                bs.body.push(statement.unwrap());
            }
            self.next_token();
        }

        // Printing output
        dbg!(&bs.body);
        if self.errors.len() > 0 {
            dbg!(&self.errors);
        }

        return bs;
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let statement: Option<Box<dyn Statement>> = match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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
        if token.type_matches(&self.peek_token) {
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

        if !(Token::Semicolon.type_matches(&self.current_token)) {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let return_value = Box::new(expressions::ExpressionNode {
            value: "10".to_string(),
        });
        self.next_token();
        // TODO filling expression (return_value)

        let statement = statements::ReturnStatement {
            token: Token::Return,
            return_value,
        };

        if !(Token::Semicolon.type_matches(&self.current_token)) {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        // unwrap or return None
        let expression = self.parse_expression(Precedence::LOWEST).unwrap();
        let statement = statements::ExpressionStatement {
            token: self.current_token.clone(),
            expression,
        };
        if Token::Semicolon.type_matches(&self.peek_token) {
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.prefix_parse_funcs.get(&self.current_token);
        if prefix.is_none() {
            return None;
        }

        let left_expression = prefix.unwrap()();
        return Some(left_expression);
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        return ();
    }
}
