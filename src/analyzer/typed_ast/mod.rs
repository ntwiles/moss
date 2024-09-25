pub mod typed_expr;

use typed_expr::TypedExpr;

#[derive(Clone, Debug)]
pub enum TypedLiteral {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct TypedStmt {
    pub expr: TypedExpr,
}

#[derive(Clone, Debug)]
pub struct TypedFunc {
    pub stmts: Vec<TypedStmt>,
    pub is_closure: bool,
}
