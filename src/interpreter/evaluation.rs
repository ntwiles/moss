use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{
    ast::typed::{typed_expr::TypedExpr, TypedFunc, TypedLiteral},
    builtins::{BuiltinFunc, BuiltinFuncId},
    errors::runtime_error::RuntimeError,
    state::{
        control_flow::ControlFlow, control_op::ControlOp, exec_context::ExecContext,
        io_context::IoContext,
    },
};

use super::{
    apply_binary_op, apply_unary_op, mark_loop, push_binary_op, push_block, push_func_call,
    push_if_else, push_unary_op, resolved_value::ResolvedValue,
};

pub fn apply_stmt(exec: &mut ExecContext) -> ControlFlow {
    let value = exec.value_stack.last().unwrap();

    if let ResolvedValue::Void = value {
        ControlFlow::Continue
    } else {
        ControlFlow::Return
    }
}

pub fn eval_expr<R: Read, W: Write>(
    exec: &mut ExecContext,
    io: &mut IoContext<R, W>,
    builtins: &HashMap<BuiltinFuncId, BuiltinFunc<R, W>>,
    expr: TypedExpr,
) -> Result<ControlFlow, RuntimeError> {
    let control_flow = match expr {
        // Binary operations
        TypedExpr::Eq(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyEq, l, r),
        TypedExpr::Gt(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyGt, l, r),
        TypedExpr::Lt(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyLt, l, r),
        TypedExpr::Gte(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyGte, l, r),
        TypedExpr::Lte(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyLte, l, r),
        TypedExpr::Add(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyAdd, l, r),
        TypedExpr::Sub(l, r, _ty) => push_binary_op(exec, ControlOp::ApplySub, l, r),
        TypedExpr::Mult(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyMult, l, r),
        TypedExpr::Div(l, r, _ty) => push_binary_op(exec, ControlOp::ApplyDiv, l, r),

        // Unary operations
        TypedExpr::Negate(l, _ty) => push_unary_op(exec, ControlOp::ApplyNegate, *l),
        TypedExpr::Assign(i, v, _ty) => push_unary_op(exec, ControlOp::ApplyAssign(i), *v),

        // Postfix operations
        TypedExpr::FuncCall(func, _ty) => push_func_call(exec, func),

        // Control flow
        TypedExpr::IfElse(cond, then, els, _ty) => push_if_else(exec, *cond, then, els),
        TypedExpr::Block(block) => push_block(exec, io, builtins, TypedExpr::Block(block))?,
        TypedExpr::Loop(block) => mark_loop(exec, *block),
        TypedExpr::Break => ControlFlow::Break,

        // Primaries
        TypedExpr::Literal(literal, _ty) => eval_literal(exec, literal),
        TypedExpr::Identifier(ident, _ty) => eval_identifier(exec, ident)?,
        TypedExpr::FuncDeclare(func, _ty) => eval_func_declare(exec, func),
    };

    Ok(control_flow)
}

// Binary operations

pub fn apply_add(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l + r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l + r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::String(l + &r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_sub(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l - r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l - r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_mult(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l * r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l * r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_div(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l / r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l / r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_eq(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Bool(l), ResolvedValue::Bool(r)) => ResolvedValue::Bool(l == r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_gt(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l > r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l > r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_gte(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l >= r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l >= r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_lt(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l < r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l < r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_lte(exec: &mut ExecContext) -> ControlFlow {
    apply_binary_op(exec, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l <= r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l <= r),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

// Unary operations
pub fn apply_negate(exec: &mut ExecContext) -> ControlFlow {
    apply_unary_op(exec, |_scope_stack, v| match v {
        ResolvedValue::Int(int) => ResolvedValue::Int(-int),
        ResolvedValue::Float(float) => ResolvedValue::Float(-float),
        _ => unreachable!(),
    });

    ControlFlow::Continue
}

pub fn apply_assign(exec: &mut ExecContext, ident: String) -> ControlFlow {
    apply_unary_op(exec, |exec, v| {
        exec.scope_stack.insert(ident.clone(), v.clone());
        ResolvedValue::Void
    });

    ControlFlow::Continue
}

// Postfix operations
pub fn apply_func_call(exec: &mut ExecContext, args: Vec<TypedExpr>) -> ControlFlow {
    let func = match exec.value_stack.pop().unwrap() {
        ResolvedValue::Function(func) => func,
        _ => unreachable!(),
    };

    if func.is_closure {
        exec.control_stack.push(ControlOp::ApplyClosureFuncCall);
    } else {
        exec.control_stack.push(ControlOp::ApplyNonClosureFuncCall);
    }

    exec.control_stack
        .push(ControlOp::EvalBlock(*func.block.clone()));

    let func_copy = func.clone();

    for param in func.params.into_iter() {
        let (param, _ty) = param;
        exec.control_stack.push(ControlOp::ApplyBinding(param));
    }

    exec.control_stack.push(ControlOp::PushScope(func_copy));

    for arg in args.into_iter().rev() {
        exec.control_stack.push(ControlOp::EvalExpr(arg));
    }

    ControlFlow::Continue
}

pub fn apply_closure_func_call(exec: &mut ExecContext) -> ControlFlow {
    exec.scope_stack.pop_scope();

    ControlFlow::Continue
}

pub fn apply_non_closure_func_call(exec: &mut ExecContext) -> ControlFlow {
    exec.scope_stack.restore_previous_stack();

    ControlFlow::Continue
}

// Primaries
pub fn eval_literal(exec: &mut ExecContext, literal: TypedLiteral) -> ControlFlow {
    match literal {
        TypedLiteral::Int(int) => exec.value_stack.push(ResolvedValue::Int(int)),
        TypedLiteral::Float(float) => exec.value_stack.push(ResolvedValue::Float(float)),
        TypedLiteral::String(string) => exec.value_stack.push(ResolvedValue::String(string)),
        TypedLiteral::Bool(boolean) => exec.value_stack.push(ResolvedValue::Bool(boolean)),
    }

    ControlFlow::Continue
}

pub fn eval_identifier(exec: &mut ExecContext, ident: String) -> Result<ControlFlow, RuntimeError> {
    let value = exec.scope_stack.lookup(&ident)?;
    exec.value_stack.push(value.clone());

    Ok(ControlFlow::Continue)
}

pub fn eval_func_declare(exec: &mut ExecContext, func: TypedFunc) -> ControlFlow {
    exec.value_stack.push(ResolvedValue::Function(func));

    ControlFlow::Continue
}
