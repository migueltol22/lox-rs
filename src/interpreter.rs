use crate::{parser::{Expr, Literal, GroupingExpr, UnaryExpr}, token::TokenType, error::LoxError};



struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        todo!()
    }

    fn evaluate(&self, expr: Expr) -> Result<RuntimeValue, LoxError> {
        match expr {
            Expr::Literal(l) => {
                match l {
                    Literal::Number(n) => Ok(RuntimeValue::Number(n)),
                    Literal::String(s) => Ok(RuntimeValue::String(s.clone())),
                    Literal::True => Ok(RuntimeValue::True),
                    Literal::False => Ok(RuntimeValue::False),
                    Literal::Nil => Ok(RuntimeValue::Nil),
                }
            },
            Expr::Grouping(g) => {
                let GroupingExpr { expression } = g;
                self.evaluate(*expression)
            },
            Expr::Unary(u) => {
                let UnaryExpr { operator, right} = u;
                let right = self.evaluate(*right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        if let RuntimeValue::Number(r) = right {
                            return Ok(RuntimeValue::Number(-r));
                        }
                        return Err(LoxError::RuntimeError());
                    }
                    _ => Err(LoxError::RuntimeError()),
                }
            },
            _ => Err(LoxError::RuntimeError()),
        }
    }

}

pub enum RuntimeValue {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}