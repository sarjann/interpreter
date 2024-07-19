use crate::ast::{Expression, Node, Statement};
use crate::expressions;
use lexer::Token;

// Program
pub struct ProgramStatement {
    pub body: Vec<Box<dyn Statement>>,
}

impl Node for ProgramStatement {
    fn literal(&self) -> String {
        return format!("[{}]", "Program".to_string());
    }
}
impl Statement for ProgramStatement {
    fn statement_node(&self) {}
}

// Let
pub struct LetStatement {
    pub token: Token,
    pub name: expressions::IdentifierExpression,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn literal(&self) -> String {
        return format!(
            "{} {} = {}",
            self.token.literal(),
            self.name.literal(),
            self.value.literal()
        );
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// Return
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn literal(&self) -> String {
        return format!("{} : {}", self.token.literal(), self.return_value.literal());
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

