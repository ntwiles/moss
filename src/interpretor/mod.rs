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
    analyzer::typed_ast::{
        typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc, TypedFuncCall, TypedStmt,
    },
    errors::runtime_error::RuntimeError,
    shared::scope_stack::ScopeStack,
};

pub fn interpret_program(
    block: TypedExpr,
    builtins: Vec<(String, TypedExpr)>,
) -> Result<ResolvedValue, RuntimeError> {
    let mut ctx = context::Context {
        control_stack: Vec::new(),
        value_stack: Vec::new(),
        scope_stack: ScopeStack::new(),
    };

    let stmts = match block {
        TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => stmts,
        _ => unreachable!(),
    };

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack
            .push(ControlOp::EvalStmt(stmt, ctx.control_stack.len()));
    }

    // Evaluate builtins
    for (ident, expr) in builtins {
        if let TypedExpr::FuncDeclare(func, _) = expr {
            let resolved = ResolvedValue::Function(func);
            ctx.scope_stack.insert(ident, resolved);
        } else {
            unreachable!();
        }
    }

    while let Some(current_op) = ctx.control_stack.pop() {
        match current_op {
            ControlOp::EvalBlock(block) => push_block(&mut ctx, block),
            ControlOp::EvalStmt(stmt, block_marker) => push_stmt(&mut ctx, stmt, block_marker),
            ControlOp::EvalExpr(e) => eval_expr(&mut ctx, e)?,
            ControlOp::ApplyStmt(block_marker) => apply_stmt(&mut ctx, block_marker),
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
            ControlOp::ApplyBinding(ident) => apply_binding(&mut ctx, ident),
            ControlOp::PushScope(func) => apply_push_scope(&mut ctx, func),
            ControlOp::ApplyIfElse(then, els) => apply_if_else(&mut ctx, then, els),
        }
    }

    Ok(ctx.value_stack.pop().unwrap())
}

fn push_stmt(ctx: &mut Context, stmt: TypedStmt, block_marker: usize) {
    ctx.control_stack.push(ControlOp::ApplyStmt(block_marker));
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

fn push_if_else(ctx: &mut Context, cond: TypedExpr, then: Box<TypedExpr>, els: Box<TypedExpr>) {
    ctx.control_stack.push(ControlOp::ApplyIfElse(*then, *els));
    ctx.control_stack.push(ControlOp::EvalExpr(cond));
}

fn push_block(ctx: &mut Context, block: TypedExpr) {
    let block = match block {
        TypedExpr::Block(block) => block,
        _ => unreachable!(),
    };

    match block {
        TypedBlock::Interpreted(stmts, _ty) => {
            for stmt in stmts.into_iter().rev() {
                ctx.control_stack
                    .push(ControlOp::EvalStmt(stmt, ctx.control_stack.len()));
            }
        }
        // TODO: Is it safe to execute right now instead of pushing to the control stack?
        TypedBlock::Builtin(params, func, _ty) => {
            let args = params
                .iter()
                .map(|param| {
                    ctx.scope_stack
                        .lookup::<RuntimeError>(param)
                        .unwrap()
                        .clone()
                })
                .collect();

            let result = func(args);
            ctx.value_stack.push(result);
        }
    };
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

fn apply_push_scope(ctx: &mut Context, func: TypedFunc) {
    if func.is_closure {
        ctx.scope_stack.push_scope()
    } else {
        ctx.scope_stack.create_new_stack()
    }
}

// This is not used by the assignment operation, but instead for things like func call args.
pub fn apply_binding(ctx: &mut Context, ident: String) {
    let value = ctx.value_stack.pop().unwrap();
    ctx.scope_stack.insert(ident.clone(), value);
}

fn apply_if_else(ctx: &mut Context, then_block: TypedExpr, else_block: TypedExpr) {
    let cond = ctx.value_stack.pop().unwrap();

    let cond_bool = match cond {
        ResolvedValue::Bool(b) => b,
        _ => unreachable!(),
    };

    let block = if cond_bool { then_block } else { else_block };

    let stmts = match block {
        TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => stmts,
        _ => unreachable!(),
    };

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack
            .push(ControlOp::EvalStmt(stmt, ctx.control_stack.len()));
    }
}
