mod control_op;
pub mod resolved_value;

use crate::analyzer::{typed_expr::TypedExpr, TypedLiteral};
use control_op::ControlOp;
use resolved_value::ResolvedValue;

pub struct Interpreter {
    control_stack: Vec<ControlOp>,
    value_stack: Vec<ResolvedValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            control_stack: Vec::new(),
            value_stack: Vec::new(),
        }
    }

    pub fn eval(&mut self, expr: TypedExpr) -> ResolvedValue {
        self.control_stack.push(ControlOp::EvalExpr(expr));

        while let Some(current_op) = self.control_stack.pop() {
            match current_op {
                ControlOp::EvalExpr(expr) => self.eval_expr(expr),
                ControlOp::ApplyAdd => self.eval_add(),
                ControlOp::ApplySub => self.eval_sub(),
                ControlOp::ApplyMult => self.eval_mult(),
                ControlOp::ApplyDiv => self.eval_div(),
                ControlOp::ApplyEq => self.eval_eq(),
                ControlOp::ApplyGt => self.eval_gt(),
                ControlOp::ApplyLt => self.eval_lt(),
                ControlOp::ApplyNegate => self.eval_negate(),
            }
        }

        self.value_stack.pop().unwrap()
    }

    fn eval_literal(&mut self, literal: TypedLiteral) {
        match literal {
            TypedLiteral::Int(int) => self.value_stack.push(ResolvedValue::Int(int)),
            TypedLiteral::Float(float) => self.value_stack.push(ResolvedValue::Float(float)),
            TypedLiteral::String(string) => self.value_stack.push(ResolvedValue::String(string)),
        }
    }

    fn push_binary_op(&mut self, op: ControlOp, left: Box<TypedExpr>, right: Box<TypedExpr>) {
        self.control_stack.push(op);
        self.control_stack.push(ControlOp::EvalExpr(*right));
        self.control_stack.push(ControlOp::EvalExpr(*left));
    }

    fn push_unary_op(&mut self, op: ControlOp, expr: TypedExpr) {
        self.control_stack.push(op);
        self.control_stack.push(ControlOp::EvalExpr(expr));
    }

    fn apply_binary_op<F>(&mut self, op: F)
    where
        F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
    {
        let right = self.value_stack.pop().unwrap();
        let left = self.value_stack.pop().unwrap();

        self.value_stack.push(op(left, right));
    }

    pub fn eval_expr(&mut self, expr: TypedExpr) {
        match expr {
            TypedExpr::Eq(l, r, _ty) => self.push_binary_op(ControlOp::ApplyEq, l, r),
            TypedExpr::Gt(l, r, _ty) => self.push_binary_op(ControlOp::ApplyGt, l, r),
            TypedExpr::Lt(l, r, _ty) => self.push_binary_op(ControlOp::ApplyLt, l, r),
            TypedExpr::Literal(literal, _) => self.eval_literal(literal),
            TypedExpr::Add(l, r, _ty) => self.push_binary_op(ControlOp::ApplyAdd, l, r),
            TypedExpr::Sub(l, r, _ty) => self.push_binary_op(ControlOp::ApplySub, l, r),
            TypedExpr::Mult(l, r, _ty) => self.push_binary_op(ControlOp::ApplyMult, l, r),
            TypedExpr::Div(l, r, _ty) => self.push_binary_op(ControlOp::ApplyDiv, l, r),
            TypedExpr::Negate(inner, _ty) => self.push_unary_op(ControlOp::ApplyNegate, *inner),
        }
    }

    fn eval_add(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l + r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l + r),
            (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::String(l + &r),
            _ => unreachable!(),
        });
    }

    fn eval_sub(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l - r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l - r),
            _ => unreachable!(),
        });
    }

    fn eval_mult(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l * r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l * r),
            _ => unreachable!(),
        });
    }

    fn eval_div(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Int(l / r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Float(l / r),
            _ => unreachable!(),
        });
    }

    fn eval_eq(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l == r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l == r),
            (ResolvedValue::String(l), ResolvedValue::String(r)) => ResolvedValue::Bool(l == r),
            _ => unreachable!(),
        });
    }

    fn eval_gt(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l > r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l > r),
            _ => unreachable!(),
        });
    }

    fn eval_lt(&mut self) {
        self.apply_binary_op(|l, r| match (l, r) {
            (ResolvedValue::Int(l), ResolvedValue::Int(r)) => ResolvedValue::Bool(l < r),
            (ResolvedValue::Float(l), ResolvedValue::Float(r)) => ResolvedValue::Bool(l < r),
            _ => unreachable!(),
        });
    }

    fn eval_negate(&mut self) {
        let inner = self.value_stack.pop().unwrap();

        self.value_stack.push(match inner {
            ResolvedValue::Int(int) => ResolvedValue::Int(-int),
            ResolvedValue::Float(float) => ResolvedValue::Float(-float),
            _ => unreachable!(),
        });
    }
}
