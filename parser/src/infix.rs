use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use lexer::{Lexer, Token, TokenType};

pub fn infix_equal() -> Box<dyn Expression> {
    return Box::new(expressions::Identifier {
        token: Token::new(TokenType::Assign, None),
        value: String::new(),
    });
}
