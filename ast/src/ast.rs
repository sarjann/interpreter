use std::fmt::{Debug, Display, Formatter};

pub trait Node {
    fn literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}
