mod control_op;
mod evaluation;
pub mod resolved_value;

use crate::analyzer::typed_expr::TypedExpr;
use control_op::ControlOp;
use evaluation::{
    eval_add, eval_div, eval_eq, eval_expr, eval_gt, eval_lt, eval_mult, eval_negate, eval_sub,
};
use resolved_value::ResolvedValue;

pub fn interpret_expr(exprs: Vec<TypedExpr>) -> ResolvedValue {
    let mut control_stack = Vec::new();
    let mut value_stack = Vec::new();

    for expr in exprs.into_iter().rev() {
        control_stack.push(ControlOp::EvalExpr(expr));
    }

    while let Some(current_op) = control_stack.pop() {
        match current_op {
            ControlOp::EvalExpr(expr) => eval_expr(&mut control_stack, &mut value_stack, expr),
            ControlOp::ApplyAdd => eval_add(&mut value_stack),
            ControlOp::ApplySub => eval_sub(&mut value_stack),
            ControlOp::ApplyMult => eval_mult(&mut value_stack),
            ControlOp::ApplyDiv => eval_div(&mut value_stack),
            ControlOp::ApplyEq => eval_eq(&mut value_stack),
            ControlOp::ApplyGt => eval_gt(&mut value_stack),
            ControlOp::ApplyLt => eval_lt(&mut value_stack),
            ControlOp::ApplyNegate => eval_negate(&mut value_stack),
        }
    }

    value_stack.pop().unwrap()
}

fn push_unary_op(control_stack: &mut Vec<ControlOp>, op: ControlOp, expr: TypedExpr) {
    control_stack.push(op);
    control_stack.push(ControlOp::EvalExpr(expr));
}

fn push_binary_op(
    control_stack: &mut Vec<ControlOp>,
    op: ControlOp,
    left: Box<TypedExpr>,
    right: Box<TypedExpr>,
) {
    control_stack.push(op);
    control_stack.push(ControlOp::EvalExpr(*right));
    control_stack.push(ControlOp::EvalExpr(*left));
}

fn apply_binary_op<F>(value_stack: &mut Vec<ResolvedValue>, op: F)
where
    F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
{
    let right = value_stack.pop().unwrap();
    let left = value_stack.pop().unwrap();

    value_stack.push(op(left, right));
}
