use std::fmt::{Display, Formatter};

pub enum Token {
    NewLine,
    WhiteSpace,
    Alphabet(String),
    Int(u8),
    AddOperator,
    MinusOperator,
    StarOperator,
    FullStop,
    SemiColon,
    Colon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Unknown,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Token::NewLine => write!(f, "Token::NewLine"),
            Token::WhiteSpace => write!(f, "Token::WhiteSpace"),
            Token::Alphabet(s) => write!(f, "Token::Alphabet({s})"),
            Token::Int(i) => write!(f, "Token::Int({i})"),
            Token::AddOperator => write!(f, "Token::AddOperator"),
            Token::MinusOperator => write!(f, "Token::MinusOperator"),
            Token::StarOperator => write!(f, "Token::StarOperator"),
            Token::FullStop => write!(f, "Token::FullStop"),
            Token::SemiColon => write!(f, "Token::SemiColon"),
            Token::Colon => write!(f, "Token::Colon"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::LeftBracket => write!(f, "Token::LeftBracket"),
            Token::RightBracket => write!(f, "Token::RightBracket"),
            Token::LeftCurlyBracket => write!(f, "Token::LeftCurlyBracket"),
            Token::RightCurlyBracket => write!(f, "Token::RightCurlyBracket"),
            Token::LeftSquareBracket => write!(f, "Token::LeftCurlyBracket"),
            Token::RightSquareBracket => write!(f, "Token::RightCurlyBracket"),
            Token::Unknown => write!(f, "Token::Unknown"),
        }
    }
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input }
    }

    fn preprocess(&self, lines: Vec<String>) -> String {
        let mut preprocessed_lines = vec![];
        for line in lines {
            if line == "" {
                continue;
            }
            let out = line.split("//").collect::<Vec<&str>>();

            preprocessed_lines.push(out.first().unwrap().to_string());
        }
        let preprocessed_string = preprocessed_lines.into_iter().collect::<String>();
        preprocessed_string
    }

    fn parse(&self, s: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        for c in s.chars() {
            let token = Lexer::get_token(c);
            tokens.push(token);
        }
        tokens
    }

    fn get_token(c: char) -> Token {
        return match c {
            '\n' => Token::NewLine,
            ' ' => Token::WhiteSpace,
            'a'..='z' | 'A'..='Z' => Token::Alphabet(c.to_string()),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Token::Int((c as u8) - 48),
            '+' => Token::AddOperator,
            '-' => Token::MinusOperator,
            '*' => Token::StarOperator,
            '.' => Token::FullStop,
            ';' => Token::SemiColon,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '(' => Token::MinusOperator,
            ')' => Token::MinusOperator,
            '{' => Token::MinusOperator,
            '}' => Token::MinusOperator,
            '[' => Token::LeftSquareBracket,
            ']' => Token::RightCurlyBracket,
            _ => Token::Unknown,
        };
    }

    pub fn print_tokens(tokens: &Vec<Token>) {
        for t in tokens {
            print!("{} ", t);
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let state = &self.input;
        let lines: Vec<String> = state
            .split("\n")
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let preprocessed: String = self.preprocess(lines);
        let tokens: Vec<Token> = self.parse(preprocessed);
        tokens
    }
}
