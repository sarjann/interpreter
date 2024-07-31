use lexer::{Lexer, Token, TokenType};

#[test]
fn tokenise() {
    let v: Vec<u8> = "let x = 10;".bytes().collect();
    let expected = vec![
        Token::new(TokenType::Let, None),
        Token::new(TokenType::Ident, Some("x".to_string())),
        Token::new(TokenType::Assign, None),
        Token::new(TokenType::Int, Some("10".to_string())),
        Token::new(TokenType::Semicolon, None),
        Token::new(TokenType::Eof, None),
    ];

    let mut lex = Lexer::new(v);
    for (_, expected_token) in expected.iter().enumerate() {
        let token = lex.next_token();
        assert_eq!(token, expected_token.clone());
    }
    assert_eq!(lex.next_token().token_type, TokenType::Eof);
}
