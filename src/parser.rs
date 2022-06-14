
// encapsulate data directly in enum or in struct?
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

struct BinaryExpr {}

struct GroupingExpr {}

struct LiteralExpr {}

struct UnaryExpr {}