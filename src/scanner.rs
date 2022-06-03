use itertools::Itertools;
use itertools::MultiPeek;
use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    curr_buf: Vec<char>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().multipeek(),
            curr_buf: Vec::new(),
            line: 1 as u32,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            if let Some(token) = self.scan_token() {
                tokens.push(token)
            } else {
                break;
            }
        }

        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        tokens
    }

    fn scan_token(&self) -> Option<Token> {
        todo!()
        // let c = self.source.peek()?;

        // match c {
        //     '(' =>
        // }
    }

    pub fn error(&self, line: u32, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn report(&self, line: u32, loc: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, msg)
    }
}
