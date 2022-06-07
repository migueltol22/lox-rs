use std::fmt;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// impl

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut literal_str = String::new();
        match &self.literal {
            Some(l) => literal_str = l.to_string(),
            None => (),
        }

        write!(
            f,
            "type: {} lexeme: {} literal: {}",
            self.token_type.to_string(),
            self.lexeme,
            literal_str,
        )
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifer(String),
    Str(String),
    Number(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    // Single Character Tokens
    LeftParens,
    RightParens,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifer,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,

    SyntaxError { error_msg: Option<&'static str>},
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
