use std::fmt::{Debug, Display, Formatter};

pub trait Node: Debug {
    fn literal(&self) -> String;
}

impl PartialEq for dyn Node {
    fn eq(&self, other: &Self) -> bool {
        self.literal() == other.literal()
    }
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

impl PartialEq for dyn Statement {
    fn eq(&self, other: &Self) -> bool {
        self.literal() == other.literal()
    }
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

impl PartialEq for dyn Expression {
    fn eq(&self, other: &Self) -> bool {
        self.literal() == other.literal()
    }
}
