use crate::analyzer::typed_expr::TypedExpr;

pub enum ControlOp {
    EvalExpr(TypedExpr),
    ApplyAdd,
    ApplySub,
    ApplyMult,
    ApplyDiv,
}
