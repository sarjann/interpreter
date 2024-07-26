use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Ident + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    Slash,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LT,
    RT,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TokenType::Illegal => write!(f, "Token::Illegal"),
            TokenType::Eof => write!(f, "Token::Eof"),
            TokenType::Ident => write!(f, "Token::Ident"),
            TokenType::Int => write!(f, "Token::Int"),
            TokenType::Assign => write!(f, "Token::Assign"),
            TokenType::Plus => write!(f, "Token::Plus"),
            TokenType::Minus => write!(f, "Token::Minus"),
            TokenType::Bang => write!(f, "Token::Bang"),
            TokenType::Asterisk => write!(f, "Token::Asterisk"),
            TokenType::Eq => write!(f, "Token::Eq"),
            TokenType::NotEq => write!(f, "Token::NotEq"),
            TokenType::Comma => write!(f, "Token::Comma"),
            TokenType::Semicolon => write!(f, "Token::Semicolon"),
            TokenType::Slash => write!(f, "Token::Slash"),
            TokenType::LParen => write!(f, "Token::LParen"),
            TokenType::RParen => write!(f, "Token::RParen"),
            TokenType::LBrace => write!(f, "Token::LBrace"),
            TokenType::RBrace => write!(f, "Token::RBrace"),
            TokenType::LT => write!(f, "Token::LT"),
            TokenType::RT => write!(f, "Token::RT"),
            TokenType::Function => write!(f, "Token::Function"),
            TokenType::Let => write!(f, "Token::Let"),
            TokenType::True => write!(f, "Token::True"),
            TokenType::False => write!(f, "Token::False"),
            TokenType::If => write!(f, "Token::If"),
            TokenType::Else => write!(f, "Token::Else"),
            TokenType::Return => write!(f, "Token::Return"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<String>) -> Self {
        let literal: String = literal.unwrap_or(String::new());
        return Token {
            token_type,
            literal,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.token_type {
            TokenType::Illegal => write!(f, "Token::Illegal"),
            TokenType::Eof => write!(f, "Token::Eof"),
            TokenType::Ident => write!(f, "Token::Ident({})", self.literal),
            TokenType::Int => write!(f, "Token::Int({})", self.literal),
            TokenType::Assign => write!(f, "Token::Assign"),
            TokenType::Plus => write!(f, "Token::Plus"),
            TokenType::Minus => write!(f, "Token::Minus"),
            TokenType::Bang => write!(f, "Token::Bang"),
            TokenType::Asterisk => write!(f, "Token::Asterisk"),
            TokenType::Eq => write!(f, "Token::Eq"),
            TokenType::NotEq => write!(f, "Token::NotEq"),
            TokenType::Comma => write!(f, "Token::Comma"),
            TokenType::Semicolon => write!(f, "Token::Semicolon"),
            TokenType::Slash => write!(f, "Token::Slash"),
            TokenType::LParen => write!(f, "Token::LParen"),
            TokenType::RParen => write!(f, "Token::RParen"),
            TokenType::LBrace => write!(f, "Token::LBrace"),
            TokenType::RBrace => write!(f, "Token::RBrace"),
            TokenType::LT => write!(f, "Token::LT"),
            TokenType::RT => write!(f, "Token::RT"),
            TokenType::Function => write!(f, "Token::Function"),
            TokenType::Let => write!(f, "Token::Let"),
            TokenType::True => write!(f, "Token::True"),
            TokenType::False => write!(f, "Token::False"),
            TokenType::If => write!(f, "Token::If"),
            TokenType::Else => write!(f, "Token::Else"),
            TokenType::Return => write!(f, "Token::Return"),
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
        self.skip_whitespace();
        let ch = self.ch.expect("Error: ch is None");
        let token = match ch {
            b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(ch),
            b'0'..=b'9' => self.read_number(ch),
            b'=' => match self.look_ahead_one() {
                Some(b'=') => {
                    self.read_char();
                    return Token::new(TokenType::Eq, None);
                }
                _ => Token::new(TokenType::Assign, None),
            },
            b'+' => Token::new(TokenType::Plus, None),
            b'-' => Token::new(TokenType::Minus, None),

            b'!' => match self.look_ahead_one() {
                Some(b'=') => {
                    self.read_char();
                    return Token::new(TokenType::NotEq, None);
                }
                _ => Token::new(TokenType::Bang, None),
            },
            b'*' => Token::new(TokenType::Asterisk, None),
            b',' => Token::new(TokenType::Comma, None),
            b';' => Token::new(TokenType::Semicolon, None),
            b'/' => Token::new(TokenType::Slash, None),
            b'(' => Token::new(TokenType::LParen, None),
            b')' => Token::new(TokenType::RParen, None),
            b'{' => Token::new(TokenType::LBrace, None),
            b'}' => Token::new(TokenType::RBrace, None),
            b'<' => Token::new(TokenType::LT, None),
            b'>' => Token::new(TokenType::RT, None),
            0 => Token::new(TokenType::Eof, None),
            _ => {
                println!("Unknown character: {}", char::from(ch));
                Token::new(TokenType::Illegal, None)
            }
        };
        self.read_char();
        token
    }

    pub fn look_ahead_one(&self) -> Option<u8> {
        let look_ahead_pos = self.read_pos + 1;
        if look_ahead_pos >= self.input.len() {
            None
        } else {
            Some(self.input[look_ahead_pos])
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            match self.ch.expect("No char found") {
                b' ' | b'\n' | b'\r' | b'\t' => self.read_char(),
                _ => break,
            }
        }
    }

    pub fn get_keyword_token(s: &str) -> Option<Token> {
        let token = match s {
            "fn" => Token::new(TokenType::Function, None),
            "let" => Token::new(TokenType::Let, None),
            "true" => Token::new(TokenType::True, None),
            "false" => Token::new(TokenType::False, None),
            "if" => Token::new(TokenType::If, None),
            "else" => Token::new(TokenType::Else, None),
            "return" => Token::new(TokenType::Return, None),
            _ => return None,
        };
        Some(token)
    }

    pub fn read_number(&mut self, ch: u8) -> Token {
        let mut ch = ch;
        let mut us: Vec<u8> = vec![];
        loop {
            match ch {
                b'0'..=b'9' => {
                    us.push(ch);
                    self.read_char();
                    ch = self.ch.expect("Missing ch (int)");
                }
                _ => break,
            }
        }

        let mut number = 0;
        for c in us {
            number = number * 10 + (c - b'0') as i32;
        }

        self.pos -= 1;
        self.read_pos -= 1;
        Token::new(TokenType::Int, Some(number.to_string()))
    }

    pub fn read_identifier(&mut self, ch: u8) -> Token {
        let mut ch = ch;
        let mut us: Vec<u8> = vec![];
        loop {
            match ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    us.push(ch);
                    self.read_char();
                    ch = self.ch.expect("Missing ch (ident)");
                }
                _ => break,
            }
        }
        let s: String =
            std::string::String::from_utf8(us).expect("Couldn't coerce identifier to utf8 String");

        let token = match Lexer::get_keyword_token(&s) {
            Some(x) => x,
            _ => Token::new(TokenType::Ident, Some(s)),
        };

        self.pos -= 1;
        self.read_pos -= 1;
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
