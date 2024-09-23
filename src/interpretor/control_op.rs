use crate::analyzer::typed_expr::TypedExpr;

pub enum ControlOp {
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
