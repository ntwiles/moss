use crate::{
    analyzer::typed_ast::{typed_expr::TypedExpr, TypedFunc, TypedLiteral},
    errors::runtime_error::RuntimeError,
};

use super::{
    apply_binary_op, apply_unary_op, context::Context, control_op::ControlOp, push_binary_op,
    push_block, push_func_call, push_if_else, push_unary_op, resolved_value::ResolvedValue,
};

pub fn apply_stmt(ctx: &mut Context, block_start_marker: usize) {
    let value = ctx.value_stack.last().unwrap().clone();

    if let ResolvedValue::Void = value {
        return;
    }

    // We have our first non-void value, so we can return early. Remove everything from the control
    // stack after the last ApplyFuncCall.
    ctx.control_stack.truncate(block_start_marker + 1);
}

pub fn eval_expr(ctx: &mut Context, expr: TypedExpr) -> Result<(), RuntimeError> {
    match expr {
        // Binary operations
        TypedExpr::Eq(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyEq, l, r),
        TypedExpr::Gt(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyGt, l, r),
        TypedExpr::Lt(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyLt, l, r),
        TypedExpr::Add(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyAdd, l, r),
        TypedExpr::Sub(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplySub, l, r),
        TypedExpr::Mult(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyMult, l, r),
        TypedExpr::Div(l, r, _ty) => push_binary_op(ctx, ControlOp::ApplyDiv, l, r),

        // Unary operations
        TypedExpr::Negate(l, _ty) => push_unary_op(ctx, ControlOp::ApplyNegate, *l),
        TypedExpr::Assign(i, v, _ty) => push_unary_op(ctx, ControlOp::ApplyAssign(i), *v),

        // Postfix operations
        TypedExpr::FuncCall(func, _ty) => push_func_call(ctx, func),

        // Control flow
        TypedExpr::IfElse(cond, then, els, _ty) => push_if_else(ctx, *cond, then, els),
        TypedExpr::Block(block) => push_block(ctx, TypedExpr::Block(block)),

        // Primaries
        TypedExpr::Literal(literal, _ty) => eval_literal(ctx, literal),
        TypedExpr::Identifier(ident, _ty) => eval_identifier(ctx, ident)?,
        TypedExpr::FuncDeclare(func, _ty) => eval_func_declare(ctx, func),
    };

    Ok(())
}

// Binary operations

pub fn apply_add(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l + r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l + r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::String(l + &r),
        _ => unreachable!(),
    });
}

pub fn apply_sub(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l - r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l - r),
        _ => unreachable!(),
    });
}

pub fn apply_mult(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l * r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l * r),
        _ => unreachable!(),
    });
}

pub fn apply_div(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l / r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l / r),
        _ => unreachable!(),
    });
}

pub fn apply_eq(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Bool(l), ResolvedValue::Bool(r)) => ResolvedValue::Bool(l == r),
        _ => unreachable!(),
    });
}

pub fn apply_gt(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l > r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l > r),
        _ => unreachable!(),
    });
}

pub fn apply_lt(ctx: &mut Context) {
    apply_binary_op(ctx, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l < r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l < r),
        _ => unreachable!(),
    });
}

// Unary operations
pub fn apply_negate(ctx: &mut Context) {
    apply_unary_op(ctx, |_scope_stack, v| match v {
        ResolvedValue::Int(int) => ResolvedValue::Int(-int),
        ResolvedValue::Float(float) => ResolvedValue::Float(-float),
        _ => unreachable!(),
    });
}

pub fn apply_assign(ctx: &mut Context, ident: String) {
    apply_unary_op(ctx, |ctx, v| {
        ctx.scope_stack.insert(ident.clone(), v.clone());
        ResolvedValue::Void
    });
}

// Postfix operations
pub fn apply_func_call(ctx: &mut Context, args: Vec<TypedExpr>) {
    let func = match ctx.value_stack.pop().unwrap() {
        ResolvedValue::Function(func) => func,
        _ => unreachable!(),
    };

    if func.is_closure {
        ctx.control_stack.push(ControlOp::ApplyClosureFuncCall);
    } else {
        ctx.control_stack.push(ControlOp::ApplyNonClosureFuncCall);
    }

    ctx.control_stack
        .push(ControlOp::EvalBlock(*func.block.clone()));

    let func_copy = func.clone();

    for param in func.params.into_iter() {
        let (param, _ty) = param;
        ctx.control_stack.push(ControlOp::ApplyBinding(param));
    }

    ctx.control_stack.push(ControlOp::PushScope(func_copy));

    for arg in args.into_iter().rev() {
        ctx.control_stack.push(ControlOp::EvalExpr(arg));
    }
}

pub fn apply_closure_func_call(ctx: &mut Context) {
    ctx.scope_stack.pop_scope();
}

pub fn apply_non_closure_func_call(ctx: &mut Context) {
    ctx.scope_stack.restore_previous_stack();
}

// Primaries
pub fn eval_literal(ctx: &mut Context, literal: TypedLiteral) {
    match literal {
        TypedLiteral::Int(int) => ctx.value_stack.push(ResolvedValue::Int(int)),
        TypedLiteral::Float(float) => ctx.value_stack.push(ResolvedValue::Float(float)),
        TypedLiteral::String(string) => ctx.value_stack.push(ResolvedValue::String(string)),
        TypedLiteral::Bool(boolean) => ctx.value_stack.push(ResolvedValue::Bool(boolean)),
    }
}

pub fn eval_identifier(ctx: &mut Context, ident: String) -> Result<(), RuntimeError> {
    let value = ctx.scope_stack.lookup(&ident)?;
    ctx.value_stack.push(value.clone());

    Ok(())
}

pub fn eval_func_declare(ctx: &mut Context, func: TypedFunc) {
    ctx.value_stack.push(ResolvedValue::Function(func));
}
