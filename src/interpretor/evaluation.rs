use crate::analyzer::{typed_expr::TypedExpr, TypedLiteral, TypedStmt};

use super::{
    apply_binary_op, apply_unary_op, context::Context, control_op::ControlOp, push_binary_op,
    push_func_call, push_unary_op, resolved_value::ResolvedValue,
};

pub fn apply_stmt(ctx: &mut Context) {
    let value = ctx.value_stack.last().unwrap().clone();

    if let ResolvedValue::Void = value {
        return;
    }

    // We have our first non-void value, so we can return early. Remove everything from the control
    // stack after the last ApplyFuncCall.

    // TODO: This is a helpful pattern, we'll want to extract it into a function.

    let mut i = ctx.control_stack.len();

    while i > 0 {
        i -= 1;
        if let ControlOp::ApplyFuncCall = ctx.control_stack[i] {
            break;
        }
    }

    ctx.control_stack.truncate(i + 1);
}

pub fn eval_expr(ctx: &mut Context, expr: TypedExpr) {
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
        TypedExpr::FuncCall(lines, _ty) => push_func_call(ctx, lines),

        // Primaries
        TypedExpr::Literal(literal, _ty) => eval_literal(ctx, literal),
        TypedExpr::Identifier(ident, _ty) => eval_identifier(ctx, ident),
        TypedExpr::FuncDeclare(lines, _ty) => eval_func_declare(ctx, lines),
    }
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
        ctx.scope_stack
            .last_mut()
            .unwrap()
            .insert(ident.clone(), v.clone());
        ResolvedValue::Void
    });
}

// Postfix operations
pub fn apply_func_call(_ctx: &mut Context) {
    // no-op for now; the function call is already resolved and the result is on the stack
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

pub fn eval_identifier(ctx: &mut Context, ident: String) {
    let value = ctx
        .scope_stack
        .iter()
        .rev()
        .find_map(|scope| scope.get(&ident))
        .unwrap()
        .clone();

    ctx.value_stack.push(value);
}

pub fn eval_func_declare(ctx: &mut Context, lines: Vec<TypedStmt>) {
    ctx.value_stack.push(ResolvedValue::Function(lines));
}
