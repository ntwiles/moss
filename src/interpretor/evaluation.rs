use std::collections::HashMap;

use crate::analyzer::{typed_expr::TypedExpr, TypedLiteral};

use super::{
    apply_binary_op, apply_unary_op, control_op::ControlOp, push_binary_op, push_func_call,
    push_unary_op, resolved_value::ResolvedValue, Scope,
};

pub fn eval_expr(
    scope_stack: &mut Vec<Scope>,
    control_stack: &mut Vec<ControlOp>,
    value_stack: &mut Vec<ResolvedValue>,
    expr: TypedExpr,
) {
    match expr {
        // Binary operations
        TypedExpr::Eq(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyEq, l, r),
        TypedExpr::Gt(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyGt, l, r),
        TypedExpr::Lt(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyLt, l, r),
        TypedExpr::Add(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyAdd, l, r),
        TypedExpr::Sub(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplySub, l, r),
        TypedExpr::Mult(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyMult, l, r),
        TypedExpr::Div(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyDiv, l, r),

        // Unary operations
        TypedExpr::Negate(l, _ty) => push_unary_op(control_stack, ControlOp::ApplyNegate, *l),
        TypedExpr::Assign(i, v, _ty) => push_unary_op(control_stack, ControlOp::ApplyAssign(i), *v),

        // Postfix operations
        TypedExpr::FuncCall(b, _ty) => push_func_call(control_stack, b),

        // Primaries
        TypedExpr::Literal(literal, _ty) => eval_literal(value_stack, literal),
        TypedExpr::Identifier(ident, _ty) => eval_identifier(scope_stack, value_stack, ident),
        TypedExpr::FuncDeclare(lines, _ty) => eval_func_declare(value_stack, lines),
    }
}

// Binary operations

pub fn eval_add(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l + r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l + r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::String(l + &r),
        _ => unreachable!(),
    });
}

pub fn eval_sub(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l - r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l - r),
        _ => unreachable!(),
    });
}

pub fn eval_mult(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l * r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l * r),
        _ => unreachable!(),
    });
}

pub fn eval_div(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l / r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l / r),
        _ => unreachable!(),
    });
}

pub fn eval_eq(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::Bool(l == r),
        (ResolvedValue::Bool(l), ResolvedValue::Bool(r)) => ResolvedValue::Bool(l == r),
        _ => unreachable!(),
    });
}

pub fn eval_gt(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l > r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l > r),
        _ => unreachable!(),
    });
}

pub fn eval_lt(value_stack: &mut Vec<ResolvedValue>) {
    apply_binary_op(value_stack, |l, r| match (l, r) {
        (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l < r),
        (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l < r),
        _ => unreachable!(),
    });
}

// Unary operations
pub fn eval_negate(scope_stack: &mut Vec<Scope>, value_stack: &mut Vec<ResolvedValue>) {
    apply_unary_op(scope_stack, value_stack, |_scope_stack, v| match v {
        ResolvedValue::Int(int) => ResolvedValue::Int(-int),
        ResolvedValue::Float(float) => ResolvedValue::Float(-float),
        _ => unreachable!(),
    });
}

pub fn eval_assign(
    value_stack: &mut Vec<ResolvedValue>,
    scope_stack: &mut Vec<HashMap<String, ResolvedValue>>,
    ident: String,
) {
    apply_unary_op(scope_stack, value_stack, |scope_stack, v| {
        scope_stack
            .last_mut()
            .unwrap()
            .insert(ident.clone(), v.clone());
        ResolvedValue::Void
    });
}

// Postfix operations
pub fn eval_func_call(_value_stack: &mut Vec<ResolvedValue>) {
    // no-op for now; the function call is already resolved and the result is on the stack
}

// Primaries
pub fn eval_literal(value_stack: &mut Vec<ResolvedValue>, literal: TypedLiteral) {
    match literal {
        TypedLiteral::Int(int) => value_stack.push(ResolvedValue::Int(int)),
        TypedLiteral::Float(float) => value_stack.push(ResolvedValue::Float(float)),
        TypedLiteral::String(string) => value_stack.push(ResolvedValue::String(string)),
        TypedLiteral::Bool(boolean) => value_stack.push(ResolvedValue::Bool(boolean)),
    }
}

pub fn eval_identifier(
    scope_stack: &mut Vec<Scope>,
    value_stack: &mut Vec<ResolvedValue>,
    ident: String,
) {
    let value = scope_stack
        .iter()
        .rev()
        .find_map(|scope| scope.get(&ident))
        .unwrap()
        .clone();

    value_stack.push(value);
}

pub fn eval_func_declare(value_stack: &mut Vec<ResolvedValue>, lines: Vec<TypedExpr>) {
    value_stack.push(ResolvedValue::Function(lines));
}
