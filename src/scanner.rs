use itertools::Itertools;
use itertools::MultiPeek;
use std::collections::HashMap;
use std::str::{Chars, FromStr};

use crate::token::Literal;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    curr_buf: Vec<char>,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().multipeek(),
            curr_buf: Vec::new(),
            line: 1 as u32,
            keywords: HashMap::from_iter([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            if let Some(token) = self.scan_token() {
                if token.token_type != TokenType::Ignore {
                    tokens.push(token);
                }
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
            ' ' | '\r' | '\t' | '\n' => self.finalize_token(TokenType::Ignore),
            '"' => self.string(),
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
                    Ok(n) => Token {
                        token_type: TokenType::Number,
                        lexeme,
                        literal: Some(Literal::Number(n)),
                        line: self.line,
                    },
                    Err(_) => self.finalize_error_token(Some("Failed to parse number")),
                }
            }
            c if c.is_ascii_alphabetic() || &c == &'_' => {
                self.advance_until(|c| c.is_ascii_alphanumeric() || c == &'_');

                let lexeme = String::from_iter(self.curr_buf.drain(..));
                let token_type = match self.keywords.get(&lexeme) {
                    Some(t) => t.clone(),
                    None => TokenType::Identifer,
                };

                Token {
                    token_type,
                    lexeme,
                    literal: None,
                    line: self.line,
                }
            }
            _ => self.finalize_error_token(Some("Unexpected character.")),
        };

        Some(token)
    }

    fn string(&mut self) -> Token {
        if let Some(_) = self.advance_until(|c| c != &'"') {
            // Consume last '"'
            self.advance();
            let lexeme = String::from_iter(self.curr_buf.drain(..))
                .trim_matches('"')
                .to_string();
            Token {
                token_type: TokenType::String,
                lexeme: lexeme.clone(),
                literal: Some(Literal::Str(lexeme.clone())),
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
        F: Fn(&char) -> bool,
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
