#[derive(Debug)]
pub enum Expr {
    // Binary operations
    Eq(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    // Unary operations
    Negate(Box<Expr>),
    Assignment(String, Box<Expr>),

    // Primaries
    Literal(Literal),
    Identifier(String),
    Function(Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}
