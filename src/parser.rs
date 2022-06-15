use crate::token::Token;

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
    Number(f32),
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
