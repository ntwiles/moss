use crate::analyzer::typed_ast::{typed_expr::TypedExpr, TypedFunc, TypedStmt};

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
    ApplyFuncCall(Vec<TypedExpr>),
    ApplyClosureFuncCall,
    ApplyNonClosureFuncCall,

    // Unary operations
    ApplyNegate,
    ApplyAssign(String),

    // Scope
    PushScope(TypedFunc),
}
