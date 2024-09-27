#[derive(Clone, Debug)]
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

    // Postfix operations
    FuncCall(FuncCall),

    // Control flow
    IfElse(Box<Expr>, Vec<Stmt>, Vec<Stmt>),

    // Primaries
    Literal(Literal),
    Identifier(String),
    FuncDeclare(FuncDeclare),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct FuncCall {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct FuncDeclare {
    pub params: Vec<(String, String)>,
    pub stmts: Vec<Stmt>,
    pub is_closure: bool,
}
