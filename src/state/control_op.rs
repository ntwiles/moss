use crate::ast::typed::{typed_expr::TypedExpr, TypedStmt};

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

    // Control flow
    ApplyIf(TypedExpr),
    ApplyIfElse(TypedExpr, TypedExpr),
    PushLoop(TypedExpr),

    // Unary operations
    ApplyAssignment(String),
    ApplyNegate,
    ApplyDeclaration(String, bool),

    // Scope
    ApplyBinding(String),
    PushScope { create_new_stack: bool },
    PopScope { restore_previous_stack: bool },

    // Markers
    MarkLoopStart,
    MarkBlockStart,

    // Post-evaluation construction
    ApplyList(usize),
}
