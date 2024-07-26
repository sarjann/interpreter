use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token};
use std::collections::HashMap;

pub fn infix_equal() -> Box<dyn Expression> {
    return Box::new(expressions::IdentifierExpression {
        token: Token::Assign,
    });
}