use crate::ast::{Expression, Node};
use lexer::Token;

// Identifier
#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}

// IntegerLiteral
#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

// Prefix
#[derive(Debug)]
pub struct Prefix {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Node for Prefix {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for Prefix {
    fn expression_node(&self) {}
}

// Infix
#[derive(Debug)]
pub struct Infix {
    pub token: Token,
    pub operator: String,
    pub left: Option<Box<dyn Expression>>,
    pub right: Option<Box<dyn Expression>>,
}

impl Node for Infix {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for Infix {
    fn expression_node(&self) {}
}
