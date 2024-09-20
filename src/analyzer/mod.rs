pub mod ty;
pub mod typed_expr;

use crate::ast::Expr;
use ty::Type;
use typed_expr::TypedExpr;

#[derive(Debug)]
pub enum TypedLiteral {
    Int(i32),
    Float(f64),
    String(String),
}

pub fn analyze_expr(expr: &Expr) -> TypedExpr {
    match expr {
        Expr::Eq(left, right) => analyze_eq(left, right),
        Expr::Gt(left, right) => analyze_gt(left, right),
        Expr::Lt(left, right) => analyze_lt(left, right),
        Expr::Add(left, right) => analyze_add(left, right),
        Expr::Sub(left, right) => analyze_sub(left, right),
        Expr::Mult(left, right) => analyze_mult(left, right),
        Expr::Div(left, right) => analyze_div(left, right),
        Expr::Literal(literal) => analyze_literal(literal),
        Expr::Negate(inner) => analyze_negate(inner),
    }
}

fn analyze_eq(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for equality comparison");
    }

    TypedExpr::Eq(Box::new(left), Box::new(right), Type::Bool)
}

fn analyze_gt(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for greater than comparison");
    }

    // TODO: Support gt for strings.
    if left.ty() != Type::Int && left.ty() != Type::Float {
        panic!("Invalid types for greater than comparison");
    }

    TypedExpr::Gt(Box::new(left), Box::new(right), Type::Bool)
}

fn analyze_lt(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for less than comparison");
    }

    // TODO: Support lt for strings.
    if left.ty() != Type::Int && left.ty() != Type::Float {
        panic!("Invalid types for less than comparison");
    }

    TypedExpr::Lt(Box::new(left), Box::new(right), Type::Bool)
}

fn analyze_add(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for addition");
    }

    if left.ty() != Type::Int && left.ty() != Type::Float && left.ty() != Type::String {
        panic!("Invalid types for addition");
    }

    let ty = left.ty();
    TypedExpr::Add(Box::new(left), Box::new(right), ty)
}

fn analyze_sub(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for subtraction");
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        panic!("Invalid types for subtraction");
    }

    let ty = left.ty();
    TypedExpr::Sub(Box::new(left), Box::new(right), ty)
}

fn analyze_mult(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for multiplication");
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        panic!("Invalid types for multiplication");
    }

    let ty = left.ty();
    TypedExpr::Mult(Box::new(left), Box::new(right), ty)
}

fn analyze_div(left: &Expr, right: &Expr) -> TypedExpr {
    let left = analyze_expr(left);
    let right = analyze_expr(right);

    if left.ty() != right.ty() {
        panic!("Invalid types for division");
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        panic!("Invalid types for division");
    }

    let ty = left.ty();
    TypedExpr::Div(Box::new(left), Box::new(right), ty)
}

fn analyze_literal(literal: &crate::ast::Literal) -> TypedExpr {
    match literal {
        crate::ast::Literal::Int(int) => TypedExpr::Literal(TypedLiteral::Int(*int), Type::Int),
        crate::ast::Literal::Float(float) => {
            TypedExpr::Literal(TypedLiteral::Float(*float), Type::Float)
        }
        crate::ast::Literal::String(string) => {
            TypedExpr::Literal(TypedLiteral::String(string.clone()), Type::String)
        }
    }
}

fn analyze_negate(inner: &Expr) -> TypedExpr {
    let inner = analyze_expr(inner);

    if inner.ty() != Type::Int && inner.ty() != Type::Float {
        panic!("Invalid types for negation");
    }

    let ty = inner.ty();
    TypedExpr::Negate(Box::new(inner), ty)
}
