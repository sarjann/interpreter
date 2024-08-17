use crate::ast::{Expression, Node};
use crate::statements;
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

// Boolean
#[derive(Debug)]
pub struct Bool {
    pub token: Token,
    pub value: bool,
}

impl Node for Bool {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for Bool {
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

// If
#[derive(Debug)]
pub struct If {
    pub token: Token,
    pub condition: Option<Box<dyn Expression>>,
    pub first: Option<statements::BlockStatement>,
    pub second: Option<statements::BlockStatement>,
}

impl Node for If {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for If {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Option<Vec<Identifier>>,
    pub body: Option<statements::BlockStatement>,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Option<Box<dyn Expression>>,
    pub arguments: Option<Vec<Box<dyn Expression>>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        return format!("[{}]", self.token);
    }
}
impl Expression for CallExpression {
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
