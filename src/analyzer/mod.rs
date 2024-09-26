pub mod ty;
pub mod typed_ast;

use crate::ast::{FuncCall, FuncDeclare, Stmt};
use crate::shared::scope_stack::ScopeStack;

use super::ast::{Expr, Literal};
use super::errors::type_error::TypeError;
use ty::Type;
use typed_ast::typed_expr::TypedExpr;
use typed_ast::{TypedFunc, TypedFuncCall, TypedLiteral, TypedStmt};

pub fn analyze_program(stmts: Vec<Stmt>) -> Result<Vec<TypedStmt>, TypeError> {
    let mut scope_stack = ScopeStack::<TypedExpr>::new();

    analyze_stmts(&mut scope_stack, stmts)
}

fn analyze_stmts(
    scope_stack: &mut ScopeStack<TypedExpr>,
    stmts: Vec<Stmt>,
) -> Result<Vec<TypedStmt>, TypeError> {
    stmts
        .into_iter()
        .map(|stmt| analyze_stmt(scope_stack, stmt))
        .collect()
}

fn analyze_stmt(
    scope_stack: &mut ScopeStack<TypedExpr>,
    stmt: Stmt,
) -> Result<TypedStmt, TypeError> {
    let expr = analyze_expr(scope_stack, stmt.expr)?;

    Ok(TypedStmt { expr })
}

fn analyze_expr(
    scope_stack: &mut ScopeStack<TypedExpr>,
    expr: Expr,
) -> Result<TypedExpr, TypeError> {
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
        Expr::FuncDeclare(func) => analyze_func_declare(scope_stack, func),
        Expr::FuncCall(call) => analyze_func_call(scope_stack, call),
    }
}

// Binary operations

fn analyze_eq(
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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
    scope_stack: &mut ScopeStack<TypedExpr>,
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

fn analyze_negate(
    scope_stack: &mut ScopeStack<TypedExpr>,
    inner: Expr,
) -> Result<TypedExpr, TypeError> {
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
    scope_stack: &mut ScopeStack<TypedExpr>,
    ident: String,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(scope_stack, value)?;

    if inner.ty() == Type::Void {
        return Err(TypeError {
            message: "Cannot assign void value".to_string(),
        });
    }

    scope_stack.insert(ident.clone(), inner.clone());

    Ok(TypedExpr::Assign(ident, Box::new(inner), Type::Void))
}

// Postfix operations

fn analyze_func_call(
    scope_stack: &mut ScopeStack<TypedExpr>,
    call: FuncCall,
) -> Result<TypedExpr, TypeError> {
    let callee = analyze_expr(scope_stack, *call.func)?;

    let callee = if let TypedExpr::Identifier(ident, _) = callee {
        scope_stack.lookup(&ident)?.clone()
    } else {
        callee
    };

    if let TypedExpr::FuncDeclare(func, _) = callee {
        let call = TypedFuncCall {
            func: Box::new(func),
            args: call
                .args
                .into_iter()
                .map(|arg| analyze_expr(scope_stack, arg))
                .collect::<Result<Vec<_>, _>>()?,
        };

        Ok(TypedExpr::FuncCall(call, Type::Void))
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

fn analyze_identifier(
    scope_stack: &mut ScopeStack<TypedExpr>,
    ident: String,
) -> Result<TypedExpr, TypeError> {
    let expr = scope_stack.lookup(&ident)?;

    Ok(TypedExpr::Identifier(ident, expr.ty()))
}

fn analyze_func_declare(
    scope_stack: &mut ScopeStack<TypedExpr>,
    func: FuncDeclare,
) -> Result<TypedExpr, TypeError> {
    if func.is_closure {
        scope_stack.push_scope();
    } else {
        scope_stack.create_new_stack();
    }

    // Inject binding for the function itself to allow for recursion

    let stmts = analyze_stmts(scope_stack, func.stmts)?;

    if func.is_closure {
        scope_stack.pop_scope();
    } else {
        scope_stack.restore_previous_stack();
    }

    let func = TypedFunc {
        params: func.params,
        stmts,
        is_closure: func.is_closure,
    };

    Ok(TypedExpr::FuncDeclare(func, Type::Function))
}
