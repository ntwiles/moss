use crate::ast::typed::{typed_expr::TypedExpr, TypedFunc, TypedStmt};

#[derive(Clone, Debug)]
pub enum ControlOp {
    EvalBlock(TypedExpr),
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
    ApplyGte,
    ApplyLte,

    // Postfix operations
    ApplyFuncCall(Vec<TypedExpr>),
    ApplyClosureFuncCall,
    ApplyNonClosureFuncCall,

    // Control flow
    ApplyIf(TypedExpr),
    ApplyIfElse(TypedExpr, TypedExpr),
    PushLoop(TypedExpr),

    // Unary operations
    ApplyNegate,
    ApplyAssign(String),

    // Scope
    ApplyBinding(String),
    PushScope(TypedFunc),

    // Markers
    MarkLoopStart,
    MarkBlockStart,
}
