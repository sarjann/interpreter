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
