pub mod typed_block;
pub mod typed_expr;

use std::fmt::{Display, Formatter, Result};

use typed_block::TypedBlock;
use typed_expr::TypedExpr;

use crate::typing::Type;

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
    pub func_expr: Box<TypedExpr>,
    pub args: Vec<TypedExpr>,
}

#[derive(Clone, Debug)]
pub struct TypedFunc {
    pub params: Vec<(String, Type)>,
    pub block: Box<TypedExpr>,
    pub is_closure: bool,
}

impl Display for TypedFunc {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let param_types = self
            .params
            .iter()
            .map(|t| t.1.to_string())
            .chain(std::iter::once(self.block.ty().to_string()))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "Func<{}>", param_types)
    }
}
