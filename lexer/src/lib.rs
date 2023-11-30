use std::fmt::{Display, Formatter};

pub enum Token {
    Illegal,
    Eof,

    // Ident + Literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Token::Illegal => write!(f, "Token::Illegal"),
            Token::Eof => write!(f, "Token::Eof"),
            Token::Ident(s) => write!(f, "Token::Ident({s})"),
            Token::Int(i) => write!(f, "Token::Int({i})"),
            Token::Assign => write!(f, "Token::Assign"),
            Token::Plus => write!(f, "Token::Plus"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::Semicolon => write!(f, "Token::Semicolon"),
            Token::LParen => write!(f, "Token::LParen"),
            Token::RParen => write!(f, "Token::RParen"),
            Token::LBrace => write!(f, "Token::LBrace"),
            Token::RBrace => write!(f, "Token::RBrace"),
            Token::Function => write!(f, "Token::Function"),
            Token::Let => write!(f, "Token::Let"),
        }
    }
}

pub struct Lexer {
    pub input: Vec<u8>,
    pub pos: usize,
    pub read_pos: usize,
    pub ch: Option<u8>,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }
    pub fn next_token(&mut self) -> Token {
        let ch = self.ch.expect("Error: ch is None");
        let token = match ch {
            b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(ch),
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }

    pub fn get_keyword_token(s: &str) -> Option<Token> {
        let token = match s {
            "let" => Token::Let,
            "fn" => Token::Function,
            _ => return None,
        };
        Some(token)
    }

    pub fn read_identifier(&mut self, ch: u8) -> Token {
        let mut ch = ch;
        let mut us: Vec<u8> = vec![];
        loop {
            match ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    us.push(ch);
                    self.read_char();
                    ch = self.ch.expect("Missing ch");
                }
                _ => break,
            }
        }
        let s: String =
            std::string::String::from_utf8(us).expect("Couldn't coerce identifier to utf8 String");

        let token = match Lexer::get_keyword_token(&s) {
            Some(x) => x,
            _ => Token::Ident(s),
        };
        token
    }

    pub fn read_char(&mut self) {
        let read_pos = self.read_pos;
        if read_pos >= self.input.len() {
            self.ch = Some(0);
        } else {
            self.ch = Some(self.input[read_pos]);
        }
        self.pos = self.read_pos;
        self.read_pos = read_pos + 1;
    }
}
