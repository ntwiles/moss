use crate::{
    ast::{
        typed::{typed_expr::TypedExpr, TypedLiteral},
        untyped::Expr,
    },
    errors::type_error::TypeError,
    scopes::{scope::Scope, scope_stack::ScopeStack},
    typing::{Type, TypeBinding},
};

use super::{analyze_expr, scope_entry::AnalyzerScopeEntry};

pub fn analyze_binary_op(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    expr: Expr,
) -> Result<TypedExpr, TypeError> {
    use Expr::*;

    match expr {
        Eq(left, right) => analyze_eq(value_scope_stack, type_scope, *left, *right),
        Gt(left, right) => analyze_gt(value_scope_stack, type_scope, *left, *right),
        Lt(left, right) => analyze_lt(value_scope_stack, type_scope, *left, *right),
        Gte(left, right) => analyze_gte(value_scope_stack, type_scope, *left, *right),
        Lte(left, right) => analyze_lte(value_scope_stack, type_scope, *left, *right),
        Add(left, right) => analyze_add(value_scope_stack, type_scope, *left, *right),
        Sub(left, right) => analyze_sub(value_scope_stack, type_scope, *left, *right),
        Mult(left, right) => analyze_mult(value_scope_stack, type_scope, *left, *right),
        Div(left, right) => analyze_div(value_scope_stack, type_scope, *left, *right),
        Modulo(left, right) => analyze_modulo(value_scope_stack, type_scope, *left, *right),
        _ => unimplemented!(),
    }
}

fn analyze_eq(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "==".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    Ok(TypedExpr::Eq(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_gt(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            ">".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    // TODO: Support gt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            ">".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    Ok(TypedExpr::Gt(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_gte(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            ">=".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    // TODO: Support gt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            ">=".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    Ok(TypedExpr::Gte(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_lt(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "<".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    // TODO: Support lt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "<".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    Ok(TypedExpr::Lt(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_lte(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "<=".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    // TODO: Support lt for strings?
    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "<=".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    Ok(TypedExpr::Lte(Box::new(left), Box::new(right), Type::Bool))
}

fn analyze_add(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "+".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if left.ty() != Type::Int && left.ty() != Type::Float && left.ty() != Type::Str {
        return Err(TypeError::BinaryOpWrongTypes(
            "+".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    let ty = left.ty();
    Ok(TypedExpr::Add(Box::new(left), Box::new(right), ty))
}

fn analyze_sub(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "-".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "-".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    let ty = left.ty();
    Ok(TypedExpr::Sub(Box::new(left), Box::new(right), ty))
}

fn analyze_mult(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "*".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "*".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    let ty = left.ty();
    Ok(TypedExpr::Mult(Box::new(left), Box::new(right), ty))
}

fn analyze_div(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "/".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "/".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if let TypedExpr::Literal(TypedLiteral::Int(0), _) = right {
        return Err(TypeError::DivisionZero);
    }

    let ty = left.ty();
    Ok(TypedExpr::Div(Box::new(left), Box::new(right), ty))
}

fn analyze_modulo(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    left: Expr,
    right: Expr,
) -> Result<TypedExpr, TypeError> {
    let left = analyze_expr(value_scope_stack, type_scope, &None, left)?;
    let right = analyze_expr(value_scope_stack, type_scope, &None, right)?;

    if left.ty() != right.ty() {
        return Err(TypeError::BinaryOpWrongTypes(
            "%".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if left.ty() != Type::Int && left.ty() != Type::Float {
        return Err(TypeError::BinaryOpWrongTypes(
            "%".to_string(),
            left.ty(),
            right.ty(),
        ));
    }

    if let TypedExpr::Literal(TypedLiteral::Int(0), _) = right {
        return Err(TypeError::DivisionZero);
    }

    let ty = left.ty();
    Ok(TypedExpr::Modulo(Box::new(left), Box::new(right), ty))
}
