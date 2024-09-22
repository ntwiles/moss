pub mod ty;
pub mod typed_expr;

use super::ast::{Expr, Literal};
use super::errors::type_error::TypeError;
use ty::Type;
use typed_expr::TypedExpr;

#[derive(Debug)]
pub enum TypedLiteral {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

pub fn analyze_exprs(exprs: Vec<Expr>) -> Result<Vec<TypedExpr>, TypeError> {
    exprs.into_iter().map(|expr| analyze_expr(expr)).collect()
}

fn analyze_expr(expr: Expr) -> Result<TypedExpr, TypeError> {
    match expr {
        Expr::Eq(left, right) => analyze_eq(*left, *right),
        Expr::Gt(left, right) => analyze_gt(*left, *right),
        Expr::Lt(left, right) => analyze_lt(*left, *right),
        Expr::Add(left, right) => analyze_add(*left, *right),
        Expr::Sub(left, right) => analyze_sub(*left, *right),
        Expr::Mult(left, right) => analyze_mult(*left, *right),
        Expr::Div(left, right) => analyze_div(*left, *right),
        Expr::Literal(literal) => analyze_literal(literal),
        Expr::Negate(inner) => analyze_negate(*inner),
        Expr::Assignment(ident, value) => analyze_assign(ident, *value),
    }
}

fn analyze_eq(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for == comparison: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    Ok(TypedExpr::Eq(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_gt(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for > comparison: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    // TODO: Support gt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError {
            message: format!(
                "Invalid types for > comparison: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    Ok(TypedExpr::Gt(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_lt(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for < comparison: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    // TODO: Support lt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError {
            message: format!(
                "Invalid types for < comparison: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    Ok(TypedExpr::Lt(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_add(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for + operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    if left.ty() != Type::Int && left.ty() != Type::Float && left.ty() != Type::String {
        return Err(TypeError {
            message: format!(
                "Invalid types for + operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    let ty = left.ty();
    Ok(TypedExpr::Add(Box::new(left), Box::new(right), ty))
}

fn analyze_sub(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for - operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError {
            message: format!(
                "Invalid types for - operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    let ty = left.ty();
    Ok(TypedExpr::Sub(Box::new(left), Box::new(right), ty))
}

fn analyze_mult(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for * operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError {
            message: format!(
                "Invalid types for * operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    let ty = left.ty();
    Ok(TypedExpr::Mult(Box::new(left), Box::new(right), ty))
}

fn analyze_div(left: Expr, right: Expr) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(left)?;
    let right = analyze_expr(right)?;

    if left.ty() != right.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for / operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError {
            message: format!(
                "Invalid types for / operation: {:?} != {:?}",
                left.ty(),
                right.ty()
            ),
        });
    }

    if let TypedExpr::Literal(TypedLiteral::Int(0), _) = right {
        return Err(TypeError {
            message: "Division by zero".to_string(),
        });
    }

    let ty = left.ty();
    Ok(TypedExpr::Div(Box::new(left), Box::new(right), ty))
}

fn analyze_literal(literal: Literal) -> Result<TypedExpr, TypeError> {
    Ok(match literal {
        Literal::Int(i) => TypedExpr::Literal(TypedLiteral::Int(i), Type::Int),
        Literal::Float(f) => TypedExpr::Literal(TypedLiteral::Float(f), Type::Float),
        Literal::String(s) => TypedExpr::Literal(TypedLiteral::String(s), Type::String),
        Literal::Bool(b) => TypedExpr::Literal(TypedLiteral::Bool(b), Type::Bool),
    })
}

fn analyze_negate(inner: Expr) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(inner)?;

    if inner.ty() != Type::Int && inner.ty() != Type::Float {
        return Err(TypeError {
            message: format!("Invalid type for negation (-) operation: {:?}", inner.ty()),
        });
    }

    let ty = inner.ty();
    Ok(TypedExpr::Negate(Box::new(inner), ty))
}

fn analyze_assign(ident: String, value: Expr) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(value)?;

    if inner.ty() == Type::Void {
        return Err(TypeError {
            message: "Cannot assign void value".to_string(),
        });
    }

    Ok(TypedExpr::Assign(ident, Box::new(inner), Type::Void))
}
