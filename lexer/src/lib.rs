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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Token::Illegal => write!(f, "Token::Illegal"),
            Token::Eof => write!(f, "Token::Eof"),
            Token::Ident(s) => write!(f, "Token::Ident({s})"),
            Token::Int(i) => write!(f, "Token::Int({i})"),
            Token::Assign => write!(f, "Token::Assign"),
            Token::Plus => write!(f, "Token::Plus"),
            Token::Minus => write!(f, "Token::Minus"),
            Token::Bang => write!(f, "Token::Bang"),
            Token::Asterisk => write!(f, "Token::Asterisk"),
            Token::Eq => write!(f, "Token::Eq"),
            Token::NotEq => write!(f, "Token::NotEq"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::Semicolon => write!(f, "Token::Semicolon"),
            Token::Slash => write!(f, "Token::Slash"),
            Token::LParen => write!(f, "Token::LParen"),
            Token::RParen => write!(f, "Token::RParen"),
            Token::LBrace => write!(f, "Token::LBrace"),
            Token::RBrace => write!(f, "Token::RBrace"),
            Token::LT => write!(f, "Token::LT"),
            Token::RT => write!(f, "Token::RT"),

            // Keywords
            Token::Function => write!(f, "Token::Function"),
            Token::Let => write!(f, "Token::Let"),
            Token::True => write!(f, "Token::True"),
            Token::False => write!(f, "Token::False"),
            Token::If => write!(f, "Token::If"),
            Token::Else => write!(f, "Token::Else"),
            Token::Return => write!(f, "Token::Return"),
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
                    return Token::Eq;
                }
                _ => Token::Assign,
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => match self.look_ahead_one() {
                Some(b'=') => {
                    self.read_char();
                    return Token::NotEq;
                }
                _ => Token::Bang,
            },
            b'*' => Token::Asterisk,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'/' => Token::Slash,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'<' => Token::LT,
            b'>' => Token::RT,
            0 => Token::Eof,
            _ => {
                println!("Unknown character: {}", char::from(ch));
                Token::Illegal
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
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
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
        Token::Int(number)
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
            _ => Token::Ident(s),
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
