use crate::analyzer::typed_ast::{typed_expr::TypedExpr, TypedStmt};

#[derive(Clone, Debug)]
pub enum ControlOp {
    EvalStmt(TypedStmt),
    ApplyStmt,
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
    ApplyClosureFuncCall,
    ApplyNonClosureFuncCall,

    // Unary operations
    ApplyNegate,
    ApplyAssign(String),
}
