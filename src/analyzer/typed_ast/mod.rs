pub mod typed_expr;

use typed_expr::TypedExpr;

use super::ty::Type;

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
pub struct TypedFuncCall {
    pub func: Box<TypedFunc>,
    pub args: Vec<TypedExpr>,
}

#[derive(Clone, Debug)]
pub struct TypedFunc {
    pub params: Vec<(String, Type)>,
    pub stmts: Vec<TypedStmt>,
    pub is_closure: bool,
}
