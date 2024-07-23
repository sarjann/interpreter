use lexer::{Lexer, Token};
use std::fs;

#[test]
fn tokenise() {
    let v: Vec<u8> = fs::read("tests/test_files/let.lang").expect("Couldn't open file");
    let expected = vec![
        Token::Let,
        Token::Ident("x".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lex = Lexer::new(v);
    for (_, expected_token) in expected.iter().enumerate() {
        let token = lex.next_token();
        assert_eq!(token, expected_token.clone());
    }
    assert_eq!(lex.next_token(), Token::Eof);
}
