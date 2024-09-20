use crate::analyzer::{ty::Type, typed_expr::TypedExpr, TypedLiteral};

#[derive(Debug)]
pub enum ResolvedValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    // ...other types as needed
}

pub fn eval_expr(expr: &TypedExpr) -> ResolvedValue {
    match expr {
        TypedExpr::Eq(left, right, ty) => eval_eq(left, right, ty),
        TypedExpr::Gt(left, right, ty) => eval_gt(left, right, ty),
        TypedExpr::Lt(left, right, ty) => eval_lt(left, right, ty),
        TypedExpr::Add(left, right, _ty) => {
            let left = eval_expr(left);
            let right = eval_expr(right);

            match (left, right) {
                (ResolvedValue::Int(left), ResolvedValue::Int(right)) => {
                    ResolvedValue::Int(left + right)
                }
                (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
                    ResolvedValue::Float(left + right)
                }
                (ResolvedValue::String(left), ResolvedValue::String(right)) => {
                    ResolvedValue::String(left + &right)
                }
                _ => unreachable!(),
            }
        }
        TypedExpr::Sub(left, right, _ty) => {
            let left = eval_expr(left);
            let right = eval_expr(right);

            match (left, right) {
                (ResolvedValue::Int(left), ResolvedValue::Int(right)) => {
                    ResolvedValue::Int(left - right)
                }
                (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
                    ResolvedValue::Float(left - right)
                }
                _ => unreachable!(),
            }
        }
        TypedExpr::Mult(left, right, _ty) => {
            let left = eval_expr(left);
            let right = eval_expr(right);

            match (left, right) {
                (ResolvedValue::Int(left), ResolvedValue::Int(right)) => {
                    ResolvedValue::Int(left * right)
                }
                (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
                    ResolvedValue::Float(left * right)
                }
                _ => unreachable!(),
            }
        }
        TypedExpr::Div(left, right, _ty) => {
            let left = eval_expr(left);
            let right = eval_expr(right);

            match (left, right) {
                (ResolvedValue::Int(left), ResolvedValue::Int(right)) => {
                    ResolvedValue::Int(left / right)
                }
                (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
                    ResolvedValue::Float(left / right)
                }
                _ => unreachable!(),
            }
        }
        TypedExpr::Literal(literal, _) => match literal {
            TypedLiteral::Int(int) => ResolvedValue::Int(*int),
            TypedLiteral::Float(float) => ResolvedValue::Float(*float),
            TypedLiteral::String(string) => ResolvedValue::String(string.clone()),
        },
        TypedExpr::Negate(expr, _) => {
            let expr = eval_expr(expr);
            match expr {
                ResolvedValue::Int(int) => ResolvedValue::Int(-int),
                ResolvedValue::Float(float) => ResolvedValue::Float(-float),
                _ => unreachable!(),
            }
        }
    }
}

fn eval_eq(left: &TypedExpr, right: &TypedExpr, _: &Type) -> ResolvedValue {
    let left = eval_expr(left);
    let right = eval_expr(right);

    match (left, right) {
        (ResolvedValue::Int(left), ResolvedValue::Int(right)) => ResolvedValue::Bool(left == right),
        (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
            ResolvedValue::Bool(left == right)
        }
        (ResolvedValue::String(left), ResolvedValue::String(right)) => {
            ResolvedValue::Bool(left == right)
        }
        (ResolvedValue::Bool(left), ResolvedValue::Bool(right)) => {
            ResolvedValue::Bool(left == right)
        }
        _ => unreachable!(),
    }
}

fn eval_gt(left: &TypedExpr, right: &TypedExpr, _: &Type) -> ResolvedValue {
    let left = eval_expr(left);
    let right = eval_expr(right);

    match (left, right) {
        (ResolvedValue::Int(left), ResolvedValue::Int(right)) => ResolvedValue::Bool(left > right),
        (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
            ResolvedValue::Bool(left > right)
        }
        (ResolvedValue::String(left), ResolvedValue::String(right)) => {
            ResolvedValue::Bool(left > right)
        }
        _ => unreachable!(),
    }
}

fn eval_lt(left: &TypedExpr, right: &TypedExpr, _: &Type) -> ResolvedValue {
    let left = eval_expr(left);
    let right = eval_expr(right);

    match (left, right) {
        (ResolvedValue::Int(left), ResolvedValue::Int(right)) => ResolvedValue::Bool(left < right),
        (ResolvedValue::Float(left), ResolvedValue::Float(right)) => {
            ResolvedValue::Bool(left < right)
        }
        (ResolvedValue::String(left), ResolvedValue::String(right)) => {
            ResolvedValue::Bool(left < right)
        }
        _ => unreachable!(),
    }
}
