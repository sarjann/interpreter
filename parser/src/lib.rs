use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token, TokenType};
use std::collections::HashMap;
// use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    prefix_parse_funcs: HashMap<TokenType, fn(&mut Parser) -> Option<Box<dyn Expression>>>,
    infix_parse_funcs: HashMap<
        TokenType,
        fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>,
    >,
    precedence_lookup: HashMap<TokenType, Precedence>,
}

impl Parser {
    fn get_prefix_parse_funcs() -> HashMap<TokenType, fn(&mut Parser) -> Option<Box<dyn Expression>>>
    {
        let mut prefix_parse_funcs: HashMap<
            TokenType,
            fn(&mut Parser) -> Option<Box<dyn Expression>>,
        > = HashMap::new();
        prefix_parse_funcs.insert(TokenType::Ident, Parser::parse_identifier);
        prefix_parse_funcs.insert(TokenType::Int, Parser::parse_integer_literal);
        prefix_parse_funcs.insert(TokenType::Bang, Parser::parse_prefix_expression);
        prefix_parse_funcs.insert(TokenType::Minus, Parser::parse_prefix_expression);
        return prefix_parse_funcs;
    }

    fn get_infix_parse_funcs() -> HashMap<
        TokenType,
        fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>,
    > {
        let mut infix_parse_funcs: HashMap<
            TokenType,
            fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>,
        > = HashMap::new();
        infix_parse_funcs.insert(TokenType::Plus, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::Minus, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::Slash, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::Asterisk, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::Eq, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::NotEq, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::LT, Parser::parse_infix_expression);
        infix_parse_funcs.insert(TokenType::RT, Parser::parse_infix_expression);
        return infix_parse_funcs;
    }

    // Prefix
    fn parse_prefix_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let mut expression = Box::new(expressions::Prefix {
            token: parser.current_token.clone(),
            operator: parser.current_token.literal.clone(),
            right: None,
        });

        parser.next_token();

        let right = parser.parse_expression(Precedence::PREFIX);
        expression.right = right;
        return Some(expression);
    }

    fn parse_identifier(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        return Some(Box::new(expressions::Identifier {
            token: parser.current_token.clone(),
            value: parser.current_token.literal.clone(),
        }));
    }

    fn parse_integer_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let value: i64 = parser
            .current_token
            .literal
            .parse::<i64>()
            .expect("Failed to parse integer");

        return Some(Box::new(expressions::IntegerLiteral {
            token: parser.current_token.clone(),
            value,
        }));
    }

    // Infix
    fn parse_infix_expression(
        parser: &mut Parser,
        left: Option<Box<dyn Expression>>,
    ) -> Option<Box<dyn Expression>> {
        let mut expression = expressions::Infix {
            token: parser.current_token.clone(),
            operator: parser.current_token.literal.clone(),
            left,
            right: None,
        };

        let precedence = parser.current_precedence();
        parser.next_token();
        expression.right = parser.parse_expression(precedence);
        return Some(Box::new(expression));
    }

    fn generate_precedence_lookup_table() -> HashMap<TokenType, Precedence> {
        let precedence_lookup: HashMap<TokenType, Precedence> = HashMap::from([
            (TokenType::Eq, Precedence::EQUALS),
            (TokenType::NotEq, Precedence::EQUALS),
            (TokenType::LT, Precedence::LESSGREATER),
            (TokenType::RT, Precedence::LESSGREATER),
            (TokenType::Plus, Precedence::SUM),
            (TokenType::Minus, Precedence::SUM),
            (TokenType::Slash, Precedence::PRODUCT),
            (TokenType::Asterisk, Precedence::PRODUCT),
        ]);
        return precedence_lookup;
    }

    fn peek_precedence(&self) -> Precedence {
        if self
            .precedence_lookup
            .contains_key(&self.peek_token.token_type)
        {
            return self.precedence_lookup[&self.peek_token.token_type].clone();
        } else {
            return Precedence::LOWEST;
        }
    }

    fn current_precedence(&self) -> Precedence {
        if self
            .precedence_lookup
            .contains_key(&self.current_token.token_type)
        {
            return self.precedence_lookup[&self.current_token.token_type].clone();
        } else {
            return Precedence::LOWEST;
        }
    }

    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::Illegal, None),
            peek_token: Token::new(TokenType::Illegal, None),
            errors: Vec::new(),
            prefix_parse_funcs: Parser::get_prefix_parse_funcs(),
            infix_parse_funcs: Parser::get_infix_parse_funcs(),
            precedence_lookup: Parser::generate_precedence_lookup_table(),
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

        self.next_token();
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
        let mut statement = statements::ExpressionStatement {
            token: self.current_token.clone(),
            expression: None,
        };
        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }
        statement.expression = expression;

        if TokenType::Semicolon == self.peek_token.token_type {
            self.next_token();
        }

        return Some(Box::new(statement));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix_fn = self.prefix_parse_funcs.get(&self.current_token.token_type);
        if prefix_fn.is_none() {
            return None;
        }

        let mut left = prefix_fn.unwrap()(self);

        let infix_map = self.infix_parse_funcs.clone();
        while !matches!(self.peek_token.token_type, TokenType::Semicolon)
            && ((precedence.clone() as i32) < (self.peek_precedence() as i32))
        {
            let infix_fn = infix_map.get(&self.peek_token.token_type);

            if infix_fn.is_none() {
                return left;
            }

            self.next_token();

            left = infix_fn.unwrap()(self, left);
        }
        return left;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        return ();
    }
}
