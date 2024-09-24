use crate::analyzer::{typed_expr::TypedExpr, TypedStmt};

#[derive(Clone, Debug)]
pub enum ControlOp {
    EvalLine(TypedStmt),
    ApplyLine,
    EvalExpr(TypedExpr),

    // Binary operations
    ApplyAdd,
    ApplySub,
    ApplyMult,
    ApplyDiv,
    ApplyEq,
    ApplyGt,
    ApplyLt,

    // Postfix operations
    ApplyFuncCall,

    // Unary operations
    ApplyNegate,
    ApplyAssign(String),
}
