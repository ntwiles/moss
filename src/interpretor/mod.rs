mod context;
mod control_op;
mod control_flow;
mod evaluation;
pub mod resolved_value;

use context::Context;
use control_flow::ControlFlow;
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

    // TODO: The logic below has a lot of overlap with push_block(). Should that just be called here?

    let stmts = match block {
        TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => stmts,
        _ => unreachable!(),
    };

    ctx.control_stack.push(ControlOp::MarkBlockStart);

    for stmt in stmts.into_iter().rev() {
        ctx.control_stack
            .push(ControlOp::EvalStmt(stmt));
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
        let control_flow = match current_op {
            ControlOp::EvalBlock(block) => push_block(&mut ctx, block),
            ControlOp::EvalStmt(stmt) => push_stmt(&mut ctx, stmt),
            ControlOp::EvalExpr(expr) => eval_expr(&mut ctx, expr)?,
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
            ControlOp::ApplyBinding(ident) => apply_binding(&mut ctx, ident),
            ControlOp::PushScope(func) => apply_push_scope(&mut ctx, func),
            ControlOp::ApplyIfElse(then, els) => apply_if_else(&mut ctx, then, els),
            ControlOp::PushLoop(block) => push_loop(&mut ctx, block),

            ControlOp::MarkLoopStart => ControlFlow::Continue,
            ControlOp::MarkBlockStart => ControlFlow::Continue,
        };

        if let ControlFlow::Break = control_flow {
            loop {
                let op = ctx.control_stack.pop().unwrap();

                if let ControlOp::MarkLoopStart = op {
                    break;
                }
            }
        }

        if let ControlFlow::Return = control_flow {
            loop {
                let op = ctx.control_stack.pop().unwrap();

                if let ControlOp::MarkBlockStart = op {
                    break;
                }
            }
        }
    }

    Ok(ctx.value_stack.pop().unwrap())
}

fn push_stmt(ctx: &mut Context, stmt: TypedStmt) -> ControlFlow {
    ctx.control_stack.push(ControlOp::ApplyStmt);
    ctx.control_stack.push(ControlOp::EvalExpr(stmt.expr));

    ControlFlow::Continue
}

fn push_unary_op(ctx: &mut Context, op: ControlOp, expr: TypedExpr) -> ControlFlow {
    ctx.control_stack.push(op);
    ctx.control_stack.push(ControlOp::EvalExpr(expr));

    ControlFlow::Continue
}

fn push_binary_op(ctx: &mut Context, op: ControlOp, left: Box<TypedExpr>, right: Box<TypedExpr>) -> ControlFlow {
    ctx.control_stack.push(op);
    ctx.control_stack.push(ControlOp::EvalExpr(*right));
    ctx.control_stack.push(ControlOp::EvalExpr(*left));

    ControlFlow::Continue
}

fn push_func_call(ctx: &mut Context, call: TypedFuncCall) -> ControlFlow {
    ctx.control_stack.push(ControlOp::ApplyFuncCall(call.args));
    ctx.control_stack.push(ControlOp::EvalExpr(*call.func_expr));

    ControlFlow::Continue
}

fn push_if_else(ctx: &mut Context, cond: TypedExpr, then: Box<TypedExpr>, els: Box<TypedExpr>) -> ControlFlow {
    ctx.control_stack.push(ControlOp::ApplyIfElse(*then, *els));
    ctx.control_stack.push(ControlOp::EvalExpr(cond));

    ControlFlow::Continue
}

fn push_block(ctx: &mut Context, block: TypedExpr) -> ControlFlow {
    let block = match block {
        TypedExpr::Block(block) => block,
        _ => unreachable!(),
    };

    match block {
        TypedBlock::Interpreted(stmts, _ty) => {
            ctx.control_stack.push(ControlOp::MarkBlockStart);
            for stmt in stmts.into_iter().rev() {
                ctx.control_stack
                    .push(ControlOp::EvalStmt(stmt));
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

    ControlFlow::Continue
}

fn mark_loop(ctx: &mut Context, block: TypedExpr) -> ControlFlow {
    ctx.control_stack.push(ControlOp::MarkLoopStart);
    push_loop(ctx, block)
}

fn push_loop(ctx: &mut Context, block: TypedExpr) -> ControlFlow {
    ctx.control_stack.push(ControlOp::PushLoop(block.clone()));
    ctx.control_stack.push(ControlOp::EvalBlock(block));

    ControlFlow::Continue
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

fn apply_push_scope(ctx: &mut Context, func: TypedFunc) -> ControlFlow {
    if func.is_closure {
        ctx.scope_stack.push_scope()
    } else {
        ctx.scope_stack.create_new_stack()
    }

    ControlFlow::Continue
}

// This is not used by the assignment operation, but instead for things like func call args.
pub fn apply_binding(ctx: &mut Context, ident: String) -> ControlFlow {
    let value = ctx.value_stack.pop().unwrap();
    ctx.scope_stack.insert(ident.clone(), value);

    ControlFlow::Continue
}

fn apply_if_else(ctx: &mut Context, then_block: TypedExpr, else_block: TypedExpr) -> ControlFlow {
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
            .push(ControlOp::EvalStmt(stmt));
    }

    ControlFlow::Continue
}
