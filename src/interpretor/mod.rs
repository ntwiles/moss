mod context;
mod control_op;
mod evaluation;
pub mod resolved_value;

use context::Context;
use control_op::ControlOp;
use evaluation::{
    apply_add, apply_assign, apply_closure_func_call, apply_div, apply_eq, apply_func_call,
    apply_gt, apply_lt, apply_mult, apply_negate, apply_non_closure_func_call, apply_stmt,
    apply_sub, eval_expr,
};
use resolved_value::ResolvedValue;

use crate::{
    analyzer::typed_ast::{typed_expr::TypedExpr, TypedFuncCall, TypedStmt},
    errors::runtime_error::RuntimeError,
    shared::scope_stack::ScopeStack,
};

pub fn interpret_lines(stmts: Vec<TypedStmt>) -> Result<ResolvedValue, RuntimeError> {
    let mut ctx = context::Context {
        control_stack: Vec::new(),
        value_stack: Vec::new(),
        scope_stack: ScopeStack::new(),
    };

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack.push(ControlOp::EvalStmt(stmt));
    }

    while let Some(current_op) = ctx.control_stack.pop() {
        match current_op {
            ControlOp::EvalStmt(stmt) => push_stmt(&mut ctx, stmt),
            ControlOp::EvalExpr(e) => eval_expr(&mut ctx, e)?,
            ControlOp::ApplyStmt => apply_stmt(&mut ctx),
            ControlOp::ApplyAdd => apply_add(&mut ctx),
            ControlOp::ApplySub => apply_sub(&mut ctx),
            ControlOp::ApplyMult => apply_mult(&mut ctx),
            ControlOp::ApplyDiv => apply_div(&mut ctx),
            ControlOp::ApplyEq => apply_eq(&mut ctx),
            ControlOp::ApplyGt => apply_gt(&mut ctx),
            ControlOp::ApplyLt => apply_lt(&mut ctx),
            ControlOp::ApplyNegate => apply_negate(&mut ctx),
            ControlOp::ApplyAssign(ident) => apply_assign(&mut ctx, ident),
            ControlOp::ApplyFuncCall(args) => apply_func_call(&mut ctx, args),
            ControlOp::ApplyClosureFuncCall => apply_closure_func_call(&mut ctx),
            ControlOp::ApplyNonClosureFuncCall => apply_non_closure_func_call(&mut ctx),
        }
    }

    Ok(ctx.value_stack.pop().unwrap())
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

fn push_func_call(ctx: &mut Context, call: TypedFuncCall) {
    ctx.control_stack.push(ControlOp::ApplyFuncCall(call.args));

    ctx.control_stack.push(ControlOp::EvalExpr(*call.func_expr));
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
