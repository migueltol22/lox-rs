use std::{fmt, io};

use crate::token::{Token, TokenType};

pub enum LoxError {
    ParserError(String, usize, Token),
    Io(io::Error),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoxError::ParserError(ref s, ref l, ref t) => {
                if t.token_type == TokenType::Eof {
                    write!(f, "{} at end {}", l, s)
                } else {
                    write!(f, "{} at '{}' {}", l, t.lexeme, s)
                }
            }
            LoxError::Io(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl From<io::Error> for LoxError {
    fn from(err: io::Error) -> LoxError {
        LoxError::Io(err)
    }
}
