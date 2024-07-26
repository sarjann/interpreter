use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token, TokenType};
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
    prefix_parse_funcs: HashMap<TokenType, fn() -> Box<dyn Expression>>,
    infix_parse_funcs: HashMap<TokenType, fn() -> Box<dyn Expression>>,
}

impl Parser {
    fn get_prefix_parse_funcs() -> HashMap<TokenType, fn() -> Box<dyn Expression>> {
        let mut prefix_parse_funcs: HashMap<TokenType, fn() -> Box<dyn Expression>> =
            HashMap::new();
        prefix_parse_funcs.insert(TokenType::Ident, prefix::prefix_equal);
        return prefix_parse_funcs;
    }

    fn get_infix_parse_funcs() -> HashMap<TokenType, fn() -> Box<dyn Expression>> {
        let mut infix_parse_funcs: HashMap<TokenType, fn() -> Box<dyn Expression>> = HashMap::new();
        infix_parse_funcs.insert(TokenType::Ident, infix::infix_equal);
        return infix_parse_funcs;
    }

    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::Illegal, None),
            peek_token: Token::new(TokenType::Illegal, None),
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

        while !matches!(self.current_token.token_type, TokenType::Eof) {
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
        let statement: Option<Box<dyn Statement>> = match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        return statement;
    }

    fn peek_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "Expected {} but got {} instead",
            token_type, self.peek_token
        ));
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if token_type == self.peek_token.token_type {
            self.next_token();
            return true;
        } else {
            self.peek_error(token_type);
            return false;
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        };

        let ident_token = self.current_token.clone();
        let identifier = expressions::Identifier {
            token: ident_token.clone(),
            value: ident_token.literal,
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }

        let statement = statements::LetStatement {
            token: Token::new(TokenType::Let, None),
            name: identifier,
            value: expression,
        };

        // TODO (recursive stuff)

        if !(TokenType::Semicolon == self.current_token.token_type) {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        // TODO filling expression (return_value)
        let statement = statements::ReturnStatement {
            token: Token::new(TokenType::Return, None),
            return_value: None,
        };

        self.next_token();

        if !(TokenType::Semicolon == self.current_token.token_type) {
            self.next_token();
        }
        return Some(Box::new(statement));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        // unwrap or return None
        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }

        let statement = statements::ExpressionStatement {
            token: self.current_token.clone(),
            expression,
        };
        if TokenType::Semicolon == self.peek_token.token_type {
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.prefix_parse_funcs.get(&self.current_token.token_type);
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
