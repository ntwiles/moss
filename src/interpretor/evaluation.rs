use crate::analyzer::{typed_expr::TypedExpr, TypedLiteral};

use super::{
    apply_binary_op, control_op::ControlOp, push_binary_op, push_unary_op,
    resolved_value::ResolvedValue,
};

pub fn eval_expr(
    control_stack: &mut Vec<ControlOp>,
    value_stack: &mut Vec<ResolvedValue>,
    expr: TypedExpr,
) {
    match expr {
        TypedExpr::Literal(literal, _) => eval_literal(value_stack, literal),
        TypedExpr::Eq(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyEq, l, r),
        TypedExpr::Gt(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyGt, l, r),
        TypedExpr::Lt(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyLt, l, r),
        TypedExpr::Add(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyAdd, l, r),
        TypedExpr::Sub(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplySub, l, r),
        TypedExpr::Mult(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyMult, l, r),
        TypedExpr::Div(l, r, _ty) => push_binary_op(control_stack, ControlOp::ApplyDiv, l, r),
        TypedExpr::Negate(i, _ty) => push_unary_op(control_stack, ControlOp::ApplyNegate, *i),
    }
}

pub fn eval_literal(value_stack: &mut Vec<ResolvedValue>, literal: TypedLiteral) {
    match literal {
        TypedLiteral::Int(int) => value_stack.push(ResolvedValue::Int(int)),
        TypedLiteral::Float(float) => value_stack.push(ResolvedValue::Float(float)),
        TypedLiteral::String(string) => value_stack.push(ResolvedValue::String(string)),
        TypedLiteral::Bool(boolean) => value_stack.push(ResolvedValue::Bool(boolean)),
    }
}

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

pub fn eval_negate(value_stack: &mut Vec<ResolvedValue>) {
    let inner = value_stack.pop().unwrap();

    value_stack.push(match inner {
        ResolvedValue::Int(int) => ResolvedValue::Int(-int),
        ResolvedValue::Float(float) => ResolvedValue::Float(-float),
        _ => unreachable!(),
    });
}
