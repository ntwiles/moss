mod control_op;
mod evaluation;
pub mod resolved_value;

use std::collections::HashMap;

use crate::analyzer::{typed_expr::TypedExpr, TypedStmt};
use control_op::ControlOp;
use evaluation::{
    eval_add, eval_assign, eval_div, eval_eq, eval_expr, eval_func_call, eval_gt, eval_line,
    eval_lt, eval_mult, eval_negate, eval_sub,
};
use resolved_value::ResolvedValue;

pub type Scope = HashMap<String, ResolvedValue>;

pub fn interpret_lines(lines: Vec<TypedStmt>) -> ResolvedValue {
    let mut control_stack = Vec::new();
    let mut value_stack = Vec::new();
    let mut scope_stack = Vec::<Scope>::new();

    scope_stack.push(HashMap::new());

    for line in lines.into_iter().rev() {
        control_stack.push(ControlOp::EvalLine(line));
    }

    while let Some(current_op) = control_stack.pop() {
        match current_op {
            ControlOp::EvalLine(l) => push_line(&mut control_stack, l),
            ControlOp::EvalExpr(e) => {
                eval_expr(&mut scope_stack, &mut control_stack, &mut value_stack, e)
            }
            ControlOp::ApplyLine => eval_line(&mut control_stack, &mut value_stack),
            ControlOp::ApplyAdd => eval_add(&mut value_stack),
            ControlOp::ApplySub => eval_sub(&mut value_stack),
            ControlOp::ApplyMult => eval_mult(&mut value_stack),
            ControlOp::ApplyDiv => eval_div(&mut value_stack),
            ControlOp::ApplyEq => eval_eq(&mut value_stack),
            ControlOp::ApplyGt => eval_gt(&mut value_stack),
            ControlOp::ApplyLt => eval_lt(&mut value_stack),
            ControlOp::ApplyNegate => eval_negate(&mut scope_stack, &mut value_stack),
            ControlOp::ApplyAssign(ident) => eval_assign(&mut value_stack, &mut scope_stack, ident),
            ControlOp::ApplyFuncCall => eval_func_call(&mut value_stack),
        }
    }

    value_stack.pop().unwrap()
}

fn push_line(control_stack: &mut Vec<ControlOp>, line: TypedStmt) {
    control_stack.push(ControlOp::ApplyLine);
    control_stack.push(ControlOp::EvalExpr(line.expr));
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

fn push_func_call(control_stack: &mut Vec<ControlOp>, lines: Vec<TypedStmt>) {
    control_stack.push(ControlOp::ApplyFuncCall);

    for line in lines.into_iter().rev() {
        control_stack.push(ControlOp::EvalLine(line));
    }
}

fn apply_binary_op<F>(value_stack: &mut Vec<ResolvedValue>, op: F)
where
    F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
{
    let right = value_stack.pop().unwrap();
    let left = value_stack.pop().unwrap();

    value_stack.push(op(left, right));
}

fn apply_unary_op<F>(scope_stack: &mut Vec<Scope>, value_stack: &mut Vec<ResolvedValue>, op: F)
where
    F: Fn(&mut Vec<Scope>, ResolvedValue) -> ResolvedValue,
{
    let value = value_stack.pop().unwrap();
    value_stack.push(op(scope_stack, value));
}
