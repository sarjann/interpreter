use std::any::Any;
use std::fmt::Debug;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait Node: Debug + AsAny {
    fn token_literal(&self) -> String {
        return format!("[{}]", "Program".to_string());
    }
}

impl PartialEq for dyn Node {
    fn eq(&self, other: &Self) -> bool {
        self.token_literal() == other.token_literal()
    }
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

impl PartialEq for dyn Statement {
    fn eq(&self, other: &Self) -> bool {
        self.token_literal() == other.token_literal()
    }
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

impl PartialEq for dyn Expression {
    fn eq(&self, other: &Self) -> bool {
        self.token_literal() == other.token_literal()
    }
}
