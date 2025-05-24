mod evaluation;
pub mod resolved_value;

use std::collections::HashMap;
use std::io::{Read, Write};

use crate::ast::typed::{
    typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc, TypedFuncCall, TypedStmt,
};
use crate::builtins::{BuiltinFunc, BuiltinFuncId};
use crate::errors::runtime_error::RuntimeError;
use crate::state::{
    control_flow::ControlFlow, control_op::ControlOp, exec_context::ExecContext,
    io_context::IoContext,
};

use evaluation::{
    apply_add, apply_assignment, apply_closure_func_call, apply_declaration, apply_div, apply_eq,
    apply_func_call, apply_gt, apply_gte, apply_list, apply_lt, apply_lte, apply_mult,
    apply_negate, apply_non_closure_func_call, apply_stmt, apply_sub, eval_expr,
};
use resolved_value::ResolvedValue;

pub fn interpret_program<R: Read, W: Write>(
    block: TypedExpr,
    mut exec: ExecContext,
    mut io: IoContext<R, W>,
    builtin_bindings: Vec<(String, TypedExpr)>,
    builtins: HashMap<BuiltinFuncId, BuiltinFunc<R, W>>,
) -> Result<ResolvedValue, RuntimeError> {
    // TODO: The logic below has a lot of overlap with push_block(). Should that just be called here?
    let stmts = match block {
        TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => stmts,
        _ => unreachable!(),
    };

    exec.control_stack.push(ControlOp::MarkBlockStart);

    for stmt in stmts.into_iter().rev() {
        exec.control_stack.push(ControlOp::EvalStmt(stmt));
    }

    // TODO: Inject this into the AST prior to execution instead of doing it here.
    // Evaluate builtins
    for (ident, expr) in builtin_bindings {
        if let TypedExpr::FuncDeclare(func, _) = expr {
            let resolved = ResolvedValue::Func(func);
            exec.scope_stack.insert(ident, false, resolved)?;
        } else {
            unreachable!();
        }
    }

    while let Some(current_op) = exec.control_stack.pop() {
        let control_flow = match current_op {
            ControlOp::ApplyList(size) => apply_list(&mut exec, size),
            ControlOp::EvalBlock(block) => push_block(&mut exec, &mut io, &builtins, block)?,
            ControlOp::EvalStmt(stmt) => push_stmt(&mut exec, stmt)?,
            ControlOp::EvalExpr(expr) => eval_expr(&mut exec, &mut io, &builtins, expr)?,
            ControlOp::ApplyStmt => apply_stmt(&mut exec),
            ControlOp::ApplyAdd => apply_add(&mut exec),
            ControlOp::ApplySub => apply_sub(&mut exec),
            ControlOp::ApplyMult => apply_mult(&mut exec),
            ControlOp::ApplyDiv => apply_div(&mut exec),
            ControlOp::ApplyEq => apply_eq(&mut exec),
            ControlOp::ApplyGt => apply_gt(&mut exec),
            ControlOp::ApplyLt => apply_lt(&mut exec),
            ControlOp::ApplyGte => apply_gte(&mut exec),
            ControlOp::ApplyLte => apply_lte(&mut exec),
            ControlOp::ApplyNegate => apply_negate(&mut exec)?,
            ControlOp::ApplyAssignment(ident) => apply_assignment(&mut exec, ident)?,
            ControlOp::ApplyDeclaration(ident, is_mutable) => {
                apply_declaration(&mut exec, is_mutable, ident)?
            }
            ControlOp::ApplyFuncCall(args) => apply_func_call(&mut exec, args),
            ControlOp::ApplyClosureFuncCall => apply_closure_func_call(&mut exec),
            ControlOp::ApplyNonClosureFuncCall => apply_non_closure_func_call(&mut exec),
            ControlOp::ApplyBinding(ident) => apply_binding(&mut exec, ident)?,
            ControlOp::PushScope(func) => apply_push_scope(&mut exec, func),
            ControlOp::ApplyIf(then) => apply_if(&mut exec, then),
            ControlOp::ApplyIfElse(then, els) => apply_if_else(&mut exec, then, els),
            ControlOp::PushLoop(block) => push_loop(&mut exec, block),

            ControlOp::MarkLoopStart => ControlFlow::Continue,
            ControlOp::MarkBlockStart => ControlFlow::Continue,
        };

        if let ControlFlow::Break = control_flow {
            unwind_until(&mut exec, |op| matches!(op, ControlOp::MarkLoopStart));
        };

        if let ControlFlow::Return = control_flow {
            unwind_until(&mut exec, |op| matches!(op, ControlOp::MarkBlockStart));
        };
    }

    Ok(exec.value_stack.pop().unwrap())
}

// Pop items from the control stack until the condition is met; generally when a marker is found.
fn unwind_until<F>(exec: &mut ExecContext, meets_pattern: F)
where
    F: Fn(&ControlOp) -> bool,
{
    while let Some(op) = exec.control_stack.pop() {
        if meets_pattern(&op) {
            break;
        }
    }
}

fn push_stmt(exec: &mut ExecContext, stmt: TypedStmt) -> Result<ControlFlow, RuntimeError> {
    exec.control_stack.push(ControlOp::ApplyStmt);
    exec.control_stack.push(ControlOp::EvalExpr(stmt.expr));

    Ok(ControlFlow::Continue)
}

fn push_unary_op(exec: &mut ExecContext, op: ControlOp, expr: TypedExpr) -> ControlFlow {
    exec.control_stack.push(op);
    exec.control_stack.push(ControlOp::EvalExpr(expr));

    ControlFlow::Continue
}

fn push_binary_op(
    exec: &mut ExecContext,
    op: ControlOp,
    left: Box<TypedExpr>,
    right: Box<TypedExpr>,
) -> ControlFlow {
    exec.control_stack.push(op);
    exec.control_stack.push(ControlOp::EvalExpr(*right));
    exec.control_stack.push(ControlOp::EvalExpr(*left));

    ControlFlow::Continue
}

fn push_func_call(exec: &mut ExecContext, call: TypedFuncCall) -> ControlFlow {
    exec.control_stack.push(ControlOp::ApplyFuncCall(call.args));
    exec.control_stack
        .push(ControlOp::EvalExpr(*call.func_expr));

    ControlFlow::Continue
}

fn push_if(exec: &mut ExecContext, cond: TypedExpr, then: Box<TypedExpr>) -> ControlFlow {
    exec.control_stack.push(ControlOp::ApplyIf(*then));
    exec.control_stack.push(ControlOp::EvalExpr(cond));

    ControlFlow::Continue
}

fn push_if_else(
    exec: &mut ExecContext,
    cond: TypedExpr,
    then: Box<TypedExpr>,
    els: Box<TypedExpr>,
) -> ControlFlow {
    exec.control_stack.push(ControlOp::ApplyIfElse(*then, *els));
    exec.control_stack.push(ControlOp::EvalExpr(cond));

    ControlFlow::Continue
}

fn push_block<R: Read, W: Write>(
    exec: &mut ExecContext,
    io: &mut IoContext<R, W>,
    builtins: &HashMap<BuiltinFuncId, BuiltinFunc<R, W>>,
    block: TypedExpr,
) -> Result<ControlFlow, RuntimeError> {
    let block = match block {
        TypedExpr::Block(block) => block,
        _ => unreachable!(),
    };

    match block {
        TypedBlock::Interpreted(stmts, _ty) => {
            exec.control_stack.push(ControlOp::MarkBlockStart);
            for stmt in stmts.into_iter().rev() {
                exec.control_stack.push(ControlOp::EvalStmt(stmt));
            }
        }
        // TODO: Is it safe to execute right now instead of pushing to the control stack?
        TypedBlock::Builtin(params, builtin_id, _ty) => {
            let args = params
                .iter()
                .map(|param| {
                    exec.scope_stack
                        .lookup::<RuntimeError>(param)
                        .unwrap()
                        .value
                        .clone()
                })
                .collect();

            let func = builtins.get(&builtin_id).unwrap();

            let result = func(io, args)?;
            exec.value_stack.push(result);
        }
    };

    Ok(ControlFlow::Continue)
}

fn mark_loop(exec: &mut ExecContext, block: TypedExpr) -> ControlFlow {
    exec.control_stack.push(ControlOp::MarkLoopStart);
    push_loop(exec, block)
}

fn push_loop(exec: &mut ExecContext, block: TypedExpr) -> ControlFlow {
    exec.control_stack.push(ControlOp::PushLoop(block.clone()));
    exec.control_stack.push(ControlOp::EvalBlock(block));

    ControlFlow::Continue
}

fn apply_binary_op<F>(exec: &mut ExecContext, op: F)
where
    F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
{
    let right = exec.value_stack.pop().unwrap();
    let left = exec.value_stack.pop().unwrap();

    exec.value_stack.push(op(left, right));
}

fn apply_unary_op<F>(exec: &mut ExecContext, op: F) -> Result<(), RuntimeError>
where
    F: Fn(&mut ExecContext, ResolvedValue) -> Result<ResolvedValue, RuntimeError>,
{
    let value = exec.value_stack.pop().unwrap();
    let result = op(exec, value)?;
    exec.value_stack.push(result);
    Ok(())
}

fn apply_push_scope(exec: &mut ExecContext, func: TypedFunc) -> ControlFlow {
    if func.is_closure {
        exec.scope_stack.push_scope()
    } else {
        exec.scope_stack.create_new_stack()
    }

    ControlFlow::Continue
}

// This is not used by the assignment or declaration operations, but instead for things like func call args.
pub fn apply_binding(exec: &mut ExecContext, ident: String) -> Result<ControlFlow, RuntimeError> {
    let value = exec.value_stack.pop().unwrap();
    exec.scope_stack.insert(ident.clone(), false, value)?;

    Ok(ControlFlow::Continue)
}

fn apply_if(exec: &mut ExecContext, then_block: TypedExpr) -> ControlFlow {
    let cond = exec.value_stack.pop().unwrap();

    let cond_bool = match cond {
        ResolvedValue::Bool(b) => b,
        _ => unreachable!(),
    };

    if cond_bool {
        let stmts = match then_block {
            TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => stmts,
            _ => unreachable!(),
        };

        for stmt in stmts.into_iter().rev() {
            exec.control_stack.push(ControlOp::EvalStmt(stmt));
        }
    }

    ControlFlow::Continue
}

fn apply_if_else(
    exec: &mut ExecContext,
    then_block: TypedExpr,
    else_expr: TypedExpr,
) -> ControlFlow {
    let cond = exec.value_stack.pop().unwrap();

    let cond_bool = match cond {
        ResolvedValue::Bool(b) => b,
        _ => unreachable!(),
    };

    let branch = if cond_bool { then_block } else { else_expr };

    match branch {
        TypedExpr::Block(TypedBlock::Interpreted(stmts, _ty)) => {
            for stmt in stmts.into_iter().rev() {
                exec.control_stack.push(ControlOp::EvalStmt(stmt));
            }

            ControlFlow::Continue
        }
        TypedExpr::IfElse(cond, then, els, _ty) => push_if_else(exec, *cond, then, els),
        u => panic!("{:?}", u),
    }
}
