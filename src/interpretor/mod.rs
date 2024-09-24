mod context;
mod control_op;
mod evaluation;
pub mod resolved_value;

use std::collections::HashMap;

use crate::analyzer::{typed_expr::TypedExpr, TypedStmt};
use context::Context;
use control_op::ControlOp;
use evaluation::{
    eval_add, eval_assign, eval_div, eval_eq, eval_expr, eval_func_call, eval_gt, eval_line,
    eval_lt, eval_mult, eval_negate, eval_sub,
};
use resolved_value::ResolvedValue;

pub type Scope = HashMap<String, ResolvedValue>;

pub fn interpret_lines(stmts: Vec<TypedStmt>) -> ResolvedValue {
    let mut ctx = context::Context {
        control_stack: Vec::new(),
        value_stack: Vec::new(),
        scope_stack: Vec::new(),
    };

    ctx.scope_stack.push(HashMap::new());

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack.push(ControlOp::EvalStmt(stmt));
    }

    while let Some(current_op) = ctx.control_stack.pop() {
        match current_op {
            ControlOp::EvalStmt(stmt) => push_stmt(&mut ctx, stmt),
            ControlOp::EvalExpr(e) => eval_expr(&mut ctx, e),
            ControlOp::ApplyStmt => eval_line(&mut ctx),
            ControlOp::ApplyAdd => eval_add(&mut ctx),
            ControlOp::ApplySub => eval_sub(&mut ctx),
            ControlOp::ApplyMult => eval_mult(&mut ctx),
            ControlOp::ApplyDiv => eval_div(&mut ctx),
            ControlOp::ApplyEq => eval_eq(&mut ctx),
            ControlOp::ApplyGt => eval_gt(&mut ctx),
            ControlOp::ApplyLt => eval_lt(&mut ctx),
            ControlOp::ApplyNegate => eval_negate(&mut ctx),
            ControlOp::ApplyAssign(ident) => eval_assign(&mut ctx, ident),
            ControlOp::ApplyFuncCall => eval_func_call(&mut ctx),
        }
    }

    ctx.value_stack.pop().unwrap()
}

fn push_stmt(ctx: &mut Context, stmt: TypedStmt) {
    ctx.control_stack.push(ControlOp::ApplyStmt);
    ctx.control_stack.push(ControlOp::EvalExpr(stmt.expr));
}

fn push_unary_op(ctx: &mut Context, op: ControlOp, expr: TypedExpr) {
    ctx.control_stack.push(op);
    ctx.control_stack.push(ControlOp::EvalExpr(expr));
}

fn push_binary_op(ctx: &mut Context, op: ControlOp, left: Box<TypedExpr>, right: Box<TypedExpr>) {
    ctx.control_stack.push(op);
    ctx.control_stack.push(ControlOp::EvalExpr(*right));
    ctx.control_stack.push(ControlOp::EvalExpr(*left));
}

fn push_func_call(ctx: &mut Context, stmts: Vec<TypedStmt>) {
    ctx.control_stack.push(ControlOp::ApplyFuncCall);

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack.push(ControlOp::EvalStmt(stmt));
    }
}

fn apply_binary_op<F>(ctx: &mut Context, op: F)
where
    F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
{
    let right = ctx.value_stack.pop().unwrap();
    let left = ctx.value_stack.pop().unwrap();

    ctx.value_stack.push(op(left, right));
}

fn apply_unary_op<F>(ctx: &mut Context, op: F)
where
    F: Fn(&mut Context, ResolvedValue) -> ResolvedValue,
{
    let value = ctx.value_stack.pop().unwrap();
    let result = op(ctx, value);
    ctx.value_stack.push(result);
}
