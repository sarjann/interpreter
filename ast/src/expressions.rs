use crate::ast::{Expression, Node};
use lexer::Token;

// Expression
#[derive(Debug)]
pub struct ExpressionNode {
    pub value: String,
}

impl Node for ExpressionNode {
    fn literal(&self) -> String {
        return format!("[{}]", self.value.clone());
    }
}
impl Expression for ExpressionNode {
    fn expression_node(&self) {}
}

// Identifier
#[derive(Debug)]
pub struct IdentifierExpression {
    pub token: Token,
}

impl Node for IdentifierExpression {
    fn literal(&self) -> String {
        return format!("[{}]", self.token.literal());
    }
}
impl Expression for IdentifierExpression {
    fn expression_node(&self) {}
}
