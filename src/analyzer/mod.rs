pub mod ty;
pub mod typed_expr;

use std::collections::HashMap;

use crate::ast::Line;

use super::ast::{Expr, Literal};
use super::errors::type_error::TypeError;
use ty::Type;
use typed_expr::TypedExpr;

#[derive(Clone, Debug)]
pub enum TypedLiteral {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

type Scope = HashMap<String, TypedExpr>;

pub fn analyze_program(lines: Vec<Line>) -> Result<Vec<TypedExpr>, TypeError> {
    let mut scope_stack = Vec::<Scope>::new();
    scope_stack.push(HashMap::new());

    analyze_lines(&mut scope_stack, lines)
}

fn analyze_lines(
    scope_stack: &mut Vec<Scope>,
    lines: Vec<Line>,
) -> Result<Vec<TypedExpr>, TypeError> {
    lines
        .into_iter()
        .map(|line| analyze_expr(scope_stack, line.expr))
        .collect()
}

// Binary operations

fn analyze_expr(scope_stack: &mut Vec<Scope>, expr: Expr) -> Result<TypedExpr, TypeError> {
    match expr {
        Expr::Literal(literal) => analyze_literal(literal),
        Expr::Identifier(ident) => analyze_identifier(scope_stack, ident),

        Expr::Eq(left, right) => analyze_eq(scope_stack, *left, *right),
        Expr::Gt(left, right) => analyze_gt(scope_stack, *left, *right),
        Expr::Lt(left, right) => analyze_lt(scope_stack, *left, *right),
        Expr::Add(left, right) => analyze_add(scope_stack, *left, *right),
        Expr::Sub(left, right) => analyze_sub(scope_stack, *left, *right),
        Expr::Mult(left, right) => analyze_mult(scope_stack, *left, *right),
        Expr::Div(left, right) => analyze_div(scope_stack, *left, *right),

        Expr::Negate(inner) => analyze_negate(scope_stack, *inner),
        Expr::Assignment(ident, expr) => analyze_assign(scope_stack, ident, *expr),
        Expr::FuncDeclare(lines) => analyze_func_declare(scope_stack, lines),
        Expr::FuncCall(callee) => analyze_func_call(scope_stack, *callee),
    }
}

fn analyze_eq(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_gt(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_lt(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_add(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_sub(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_mult(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

fn analyze_div(
    scope_stack: &mut Vec<Scope>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(scope_stack, left)?;
    let right = analyze_expr(scope_stack, right)?;

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

// Unary operations

fn analyze_negate(scope_stack: &mut Vec<Scope>, inner: Expr) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(scope_stack, inner)?;

    if inner.ty() != Type::Int && inner.ty() != Type::Float {
        return Err(TypeError {
            message: format!("Invalid type for negation (-) operation: {:?}", inner.ty()),
        });
    }

    let ty = inner.ty();
    Ok(TypedExpr::Negate(Box::new(inner), ty))
}

fn analyze_assign(
    scope_stack: &mut Vec<Scope>,
    ident: String,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(scope_stack, value)?;

    if inner.ty() == Type::Void {
        return Err(TypeError {
            message: "Cannot assign void value".to_string(),
        });
    }

    scope_stack
        .last_mut()
        .unwrap()
        .insert(ident.clone(), inner.clone());

    Ok(TypedExpr::Assign(ident, Box::new(inner), Type::Void))
}

// Postfix operations

fn analyze_func_call(scope_stack: &mut Vec<Scope>, callee: Expr) -> Result<TypedExpr, TypeError> {
    let callee = analyze_expr(scope_stack, callee)?;

    let callee = if let TypedExpr::Identifier(ident, _) = callee {
        scope_stack
            .iter()
            .rev()
            .find_map(|scope| scope.get(&ident))
            .cloned()
            .ok_or(TypeError {
                message: format!("Identifier {} not found", ident),
            })?
    } else {
        callee
    };

    if let TypedExpr::FuncDeclare(lines, _) = callee {
        Ok(TypedExpr::FuncCall(lines, Type::Void))
    } else {
        Err(TypeError {
            message: format!("Cannot call non-function: {:?}", callee.ty()),
        })
    }
}

// Primaries

fn analyze_literal(literal: Literal) -> Result<TypedExpr, TypeError> {
    Ok(match literal {
        Literal::Int(i) => TypedExpr::Literal(TypedLiteral::Int(i), Type::Int),
        Literal::Float(f) => TypedExpr::Literal(TypedLiteral::Float(f), Type::Float),
        Literal::String(s) => TypedExpr::Literal(TypedLiteral::String(s), Type::String),
        Literal::Bool(b) => TypedExpr::Literal(TypedLiteral::Bool(b), Type::Bool),
    })
}

fn analyze_identifier(scope_stack: &mut Vec<Scope>, ident: String) -> Result<TypedExpr, TypeError> {
    let expr = scope_stack
        .iter()
        .rev()
        .find_map(|scope| scope.get(&ident))
        .cloned()
        .ok_or(TypeError {
            message: format!("Identifier {} not found", ident),
        })?;

    Ok(TypedExpr::Identifier(ident, expr.ty()))
}

fn analyze_func_declare(
    scope_stack: &mut Vec<Scope>,
    lines: Vec<Line>,
) -> Result<TypedExpr, TypeError> {
    let analyzed = analyze_lines(scope_stack, lines)?;

    Ok(TypedExpr::FuncDeclare(analyzed, Type::Function))
}
