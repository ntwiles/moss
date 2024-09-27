use crate::analyzer::typed_ast::{typed_expr::TypedExpr, TypedFunc, TypedStmt};

#[derive(Clone, Debug)]
pub enum ControlOp {
    EvalStmt(TypedStmt, usize),
    ApplyStmt(usize),
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

    // Control flow
    ApplyIfElse(Vec<TypedStmt>, Vec<TypedStmt>),

    // Unary operations
    ApplyNegate,
    ApplyAssign(String),

    // Scope
    PushScope(TypedFunc),
}
