use crate::ast::{Expression, Node, Statement};
use crate::expressions;
use lexer::{Token, TokenType};
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

// Program
#[derive(Debug)]
pub struct ProgramStatement {
    pub body: Vec<Box<dyn Statement>>,
}

impl PartialEq for ProgramStatement {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
    }
}

impl Node for ProgramStatement {
    fn token_literal(&self) -> String {
        return format!("[{}]", "Program".to_string());
    }
}
impl Statement for ProgramStatement {
    fn statement_node(&self) {}
}

// Let
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: expressions::Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return format!("{}", self.token);
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// Return
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return format!("{}", self.token);
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

// Expression
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return format!("{}", self.token);
    }
}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}
