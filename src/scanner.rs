use itertools::Itertools;
use itertools::MultiPeek;
use std::str::{Chars, FromStr};

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
        let c = self.advance()?;

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
                    self.advance_until(|c| c != &'\n');
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
            d if d.is_ascii_digit() => {
                let next_char = self.advance_until(|c| c.is_ascii_digit());
                if next_char == Some('.') {
                    if let Some(c) = self.source.peek() {
                        if c.is_ascii_digit() {
                            self.advance();
                            self.advance_until(|c| c.is_ascii_digit());
                        }
                    }
                }
                let lexeme = String::from_iter(self.curr_buf.drain(..));
                match f64::from_str(&lexeme) {
                    Ok(_) => Token {
                        token_type: TokenType::Number,
                        lexeme,
                        literal: None,
                        line: self.line,
                    },
                    Err(_) => self.finalize_error_token(Some("Failed to parse number"))
                }
            }
            _ => self.finalize_error_token(Some("Unexpected character.")),
        };

        Some(token)
    }

    fn string(&mut self) -> Token {
        if let Some(_) = self.advance_until(|c| c != &'"') {
            let lexeme= String::from_iter(self.curr_buf.drain(..)).trim_matches('"').to_string();
            Token {
                token_type: TokenType::String,
                lexeme,
                literal: None,
                line: self.line,
            }
        } else {
            self.finalize_error_token(Some("Unterminated string."))
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.next()?;
        if c == '\n' {
            self.line += 1;
        }
        self.curr_buf.push(c);
        Some(c)
    }

   fn advance_until<F>(&mut self, f: F) -> Option<char>
   where
        F: Fn(&char) -> bool 
    {
        loop {
            let next = self.source.peek()?;
            if f(next) {
                self.advance();
            } else {
                return Some(*next);
            }
        }
    }

    fn advance_on_match(&mut self, c: char) -> bool {
        if Some(&c) == self.source.peek() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn finalize_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = String::from_iter(self.curr_buf.drain(..));
        Token {
            token_type,
            lexeme,
            literal: None,
            line: self.line,
        }
    }

    fn finalize_error_token(&mut self, msg: Option<&'static str>) -> Token {
        let token_type = TokenType::SyntaxError { error_msg: msg };
        self.finalize_token(token_type)
    }

    fn error(&self, line: u32, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&self, line: u32, loc: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, msg)
    }
}
