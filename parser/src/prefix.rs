use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token, TokenType};

pub fn prefix_equal() -> Box<dyn Expression> {
    return Box::new(expressions::Identifier {
        token: Token::new(TokenType::Eq, None),
        value: String::new(),
    });
}
