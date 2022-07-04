use crate::{
    error::LoxError,
    token::{self, Token, TokenType},
};

/*

Complete Expression Grammar

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

*/

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison()?);
            let left = Box::new(expr);
            expr = Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_token(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term()?);
            let left = Box::new(expr);

            expr = Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_token(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            let left = Box::new(expr);

            expr = Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            let left = Box::new(expr);

            expr = Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary(UnaryExpr { operator, right }));
        }

        self.primary()
    }

    // refactor to use match?
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.match_token(&vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.match_token(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.match_token(&vec![TokenType::Number]) {
            if let Some(token::Literal::Number(value)) = self.previous().literal {
                return Ok(Expr::Literal(Literal::Number(value)));
            }
        }
        if self.match_token(&vec![TokenType::String]) {
            if let Some(token::Literal::Str(value)) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::String(value.into())));
            }
        }
        if self.match_token(&vec![TokenType::LeftParens]) {
            let expression = Box::new(self.expression()?);
            self.consume(TokenType::RightParens, "Expect '(' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr { expression }));
        }

        Err(LoxError::ParserError(
            "Expect expression.".into(),
            self.curr,
            self.peek().clone(),
        ))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, LoxError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(LoxError::ParserError(
            msg.into(),
            self.curr,
            self.peek().clone(),
        ))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.curr += 1;
        }
        return self.previous();
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *token_type;
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }
}

// encapsulate data directly in enum or in struct?
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(Literal),
    Unary(UnaryExpr),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

/*

    Example of constructing Expression

    let expression = Expr::Binary(BinaryExpr{
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token{
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal::Number(123.0))),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr{
            expression: Box::new(Expr::Literal(Literal::Number(35.67)))
        }))
    });

    println!("{:?}", expression);
*/
