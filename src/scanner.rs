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
                if self.advance_on_match('=') {
                    self.finalize_token(TokenType::BangEqual)
                } else {
                    self.finalize_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.advance_on_match('=') {
                    self.finalize_token(TokenType::EqualEqual)
                } else {
                    self.finalize_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.advance_on_match('=') {
                    self.finalize_token(TokenType::LessEqual)
                } else {
                    self.finalize_token(TokenType::Less)
                }
            }
            '>' => {
                if self.advance_on_match('=') {
                    self.finalize_token(TokenType::GreaterEqual)
                } else {
                    self.finalize_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.advance_on_match('/') {
                    self.advance_until('\n');
                    self.curr_buf.clear();
                    self.scan_token()?
                } else {
                    self.finalize_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' | '\n' => {
                return None;
            }
            '"' => {
                self.string()
            }
            _ => self.finalize_error_token(Some("Unexpected character.")),
        };

        Some(token)
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.source.next()?;
        if c == '\n' {
            self.line += 1;
        }
        self.curr_buf.push(c);
        Some(c)
    }

    pub fn advance_until(&mut self, c: char) -> Option<char> {
        loop {
            let next = self.source.peek()?;
            if next != &c {
                self.advance();
            } else {
                return Some(c);
            }
        }
    }

    pub fn advance_on_match(&mut self, c: char) -> bool {
        if Some(&c) == self.source.peek() {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn finalize_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = String::from_iter(self.curr_buf.drain(..));
        Token {
            token_type,
            lexeme,
            literal: None,
            line: self.line,
        }
    }

    pub fn finalize_error_token(&mut self, msg: Option<&'static str>) -> Token {
        let token_type = TokenType::SyntaxError { error_msg: msg };
        self.finalize_token(token_type)
    }

    pub fn error(&self, line: u32, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn report(&self, line: u32, loc: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, msg)
    }
}
