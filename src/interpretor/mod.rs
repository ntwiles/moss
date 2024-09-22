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
        // Push the initial expression onto the control stack
        self.control_stack.push(ControlOp::EvalExpr(expr));

        // Main loop to evaluate the control stack
        while let Some(current_op) = self.control_stack.pop() {
            match current_op {
                ControlOp::EvalExpr(expr) => self.eval_expr(expr),
                ControlOp::ApplyAdd => self.eval_add(),
                ControlOp::ApplySub => self.eval_sub(),
                ControlOp::ApplyMult => self.eval_mult(),
                ControlOp::ApplyDiv => self.eval_div(),
            }
        }

        // Once the control stack is empty, return the final value from the value stack
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

    fn push_unary_op(&mut self, expr: TypedExpr) {
        self.control_stack.push(ControlOp::EvalExpr(expr));
    }

    fn apply_binary_op<F>(&mut self, op: F)
    where
        F: Fn(ResolvedValue, ResolvedValue) -> ResolvedValue,
    {
        // Pop two values from the value stack
        let right = self.value_stack.pop().unwrap();
        let left = self.value_stack.pop().unwrap();

        // Apply the operator and push the result
        let result = op(left, right);
        self.value_stack.push(result);
    }

    pub fn eval_expr(&mut self, expr: TypedExpr) {
        match expr {
            // TypedExpr::Eq(l, r, ty) => eval_eq(left, right, ty),
            // TypedExpr::Gt(left, right, ty) => eval_gt(left, right, ty),
            // TypedExpr::Lt(left, right, ty) => eval_lt(left, right, ty),
            TypedExpr::Literal(literal, _) => self.eval_literal(literal),
            TypedExpr::Add(l, r, _ty) => self.push_binary_op(ControlOp::ApplyAdd, l, r),
            TypedExpr::Sub(l, r, _ty) => self.push_binary_op(ControlOp::ApplySub, l, r),
            TypedExpr::Mult(l, r, _ty) => self.push_binary_op(ControlOp::ApplyMult, l, r),
            TypedExpr::Div(l, r, _ty) => self.push_binary_op(ControlOp::ApplyDiv, l, r),
            _ => unimplemented!(),
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
}
