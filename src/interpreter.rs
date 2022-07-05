use crate::{parser::{Expr, Literal, GroupingExpr, UnaryExpr, BinaryExpr}, token::{TokenType}, error::LoxError};



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
                    Literal::Boolean(b) => Ok(RuntimeValue::Boolean(b)),
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
                    },
                    TokenType::Bang => {
                        return Ok(RuntimeValue::Boolean(!right.is_truthy()));
                    },
                    _ => Err(LoxError::RuntimeError()),
                }
            },
            Expr::Binary(b) => {
                let BinaryExpr { left, operator, right } = b;
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l - r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::Slash => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l / r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::Star => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l * r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::Plus => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Number(l + r)),
                            (RuntimeValue::String(l), RuntimeValue::String(r)) => Ok(RuntimeValue::String(format!("{}{}", l, r))),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::Greater => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Boolean(l > r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Boolean(l >= r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::Less => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Boolean(l < r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::LessEqual => {
                        match (left, right) {
                            (RuntimeValue::Number(l), RuntimeValue::Number(r)) => Ok(RuntimeValue::Boolean(l <= r)),
                            _ => Err(LoxError::RuntimeError())
                        }
                    },
                    TokenType::BangEqual => Ok(RuntimeValue::Boolean(!left.is_equal(right))),
                    TokenType::EqualEqual => Ok(RuntimeValue::Boolean(left.is_equal(right))),
                    _ => Err(LoxError::RuntimeError())
                }
            },
            _ => Err(LoxError::RuntimeError()),
        }
    }

}

pub enum RuntimeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        if let RuntimeValue::Nil = self {
            return false;
        } else if let RuntimeValue::Boolean(b) = self {
            return *b;
        } else {
            return true;
        }
    }

    pub fn is_equal(&self, rhs: RuntimeValue) -> bool {
        match (self, rhs) {
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            (RuntimeValue::String(s), RuntimeValue::String(r)) => *s == r,
            (RuntimeValue::Number(s), RuntimeValue::Number(r)) => *s == r,
            (RuntimeValue::Boolean(s), RuntimeValue::Boolean(r)) => *s == r,
            (_, _) => false,
            
        }
    }
}