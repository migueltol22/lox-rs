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

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.source.next()?;

        let token = match c {
            '(' => self.finalize_token(TokenType::LeftParens),
            ')' => self.finalize_token(TokenType::RightParens),
            '{' => self.finalize_token(TokenType::LeftBrace),
            '}' => self.finalize_token(TokenType::RightBrace),
            ',' => self.finalize_token(TokenType::Comma),
            '.' => self.finalize_token(TokenType::Dot),
            '-' => self.finalize_token(TokenType::Minus),
            ';' => self.finalize_token(TokenType::SemiColon),
            '*' => self.finalize_token(TokenType::Star),
            '!' => {
                if self.match_on_next_char('=') {
                    self.finalize_token(TokenType::BangEqual)
                } else {
                    self.finalize_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_on_next_char('=') {
                    self.finalize_token(TokenType::EqualEqual)
                } else {
                    self.finalize_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_on_next_char('=') {
                    self.finalize_token(TokenType::LessEqual)
                } else {
                    self.finalize_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_on_next_char('=') {
                    self.finalize_token(TokenType::GreaterEqual)
                } else {
                    self.finalize_token(TokenType::Greater)
                }
            },
            _ => self.finalize_error_token(None),
        };

        Some(token)
    }

    pub fn match_on_next_char(&mut self, c: char) -> bool {
        if Some(&c) == self.source.peek() {
            // maybe add this to buffer
            self.source.next();
            true
        } else {
            false
        }
    }

    pub fn finalize_token(&self, token_type: TokenType) -> Token {
        todo!()
    }

    pub fn finalize_error_token(&self, msg: Option<&'static str>) -> Token {
        todo!()
    }

    pub fn error(&self, line: u32, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn report(&self, line: u32, loc: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, msg)
    }
}
