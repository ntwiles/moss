mod scope_entry;
pub mod ty;
pub mod typed_ast;

use crate::ast::{FuncCall, FuncDeclare, Stmt};
use crate::shared::scope_stack::ScopeStack;

use super::ast::{Expr, Literal};
use super::errors::type_error::TypeError;
use scope_entry::ScopeEntry;
use ty::Type;
use typed_ast::typed_expr::TypedExpr;
use typed_ast::{TypedFunc, TypedFuncCall, TypedLiteral, TypedStmt};

pub fn analyze_program(stmts: Expr) -> Result<TypedExpr, TypeError> {
    let mut scope_stack = ScopeStack::<ScopeEntry>::new();

    analyze_block(&mut scope_stack, stmts)
}

fn analyze_stmts(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    stmts: Vec<Stmt>,
) -> Result<Vec<TypedStmt>, TypeError> {
    stmts
        .into_iter()
        .map(|stmt| analyze_stmt(scope_stack, stmt))
        .collect()
}

fn analyze_stmt(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    stmt: Stmt,
) -> Result<TypedStmt, TypeError> {
    let expr = analyze_expr(scope_stack, stmt.expr)?;

    Ok(TypedStmt { expr })
}

fn analyze_expr(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    expr: Expr,
) -> Result<TypedExpr, TypeError> {
    match expr {
        Expr::Block(stmts) => analyze_block(scope_stack, Expr::Block(stmts)),
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
        Expr::IfElse(expr, then, els) => analyze_if_else(scope_stack, *expr, *then, *els),
    }
}

// Binary operations

fn analyze_eq(
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
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
    scope_stack: &mut ScopeStack<ScopeEntry>,
    ident: String,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(scope_stack, value)?;

    if inner.ty() == Type::Void {
        return Err(TypeError {
            message: "Cannot assign void value".to_string(),
        });
    }

    let entry = ScopeEntry::TypedExpr(inner.clone());
    scope_stack.insert(ident.clone(), entry);

    Ok(TypedExpr::Assign(ident, Box::new(inner), Type::Void))
}

// Postfix operations

fn analyze_func_call(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    call: FuncCall,
) -> Result<TypedExpr, TypeError> {
    let callee = analyze_expr(scope_stack, *call.func)?;

    let args = call
        .args
        .into_iter()
        .map(|arg| analyze_expr(scope_stack, arg))
        .collect::<Result<Vec<_>, _>>()?;

    if let Type::Function(inner_types) = callee.ty() {
        let mut inner_types = inner_types.clone();

        let return_type = inner_types.pop().unwrap();
        let param_types = inner_types;

        if param_types.len() != args.len() {
            return Err(TypeError {
                // TODO: Impl Display for types here, right now it's just debug printing a tuple.
                message: format!("Called function with wrong number of args.\n\tExpected: {:?}\n\tReceived: {:?}", param_types, args)
            });
        }

        for (param_type, arg) in param_types.clone().into_iter().zip(args.clone()) {
            if arg.ty() != param_type {
                // TODO: Impl Display for types here, right now it's just debug printing a tuple.
                return Err(TypeError { message: format!("Called function with incorrect arg types.\n\tExpected: {:?}\n\tReceived: {:?}", param_types, args)});
            }
        }

        let func_call = TypedFuncCall {
            func_expr: Box::new(callee),
            args,
        };

        Ok(TypedExpr::FuncCall(func_call, return_type))
    } else {
        return Err(TypeError {
            message: format!("Cannot call non-function: {:?}", callee.ty()),
        });
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

// TODO: Should this resolve the identifier from the scope and error of it's just a Type?
fn analyze_identifier(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    ident: String,
) -> Result<TypedExpr, TypeError> {
    let expr = scope_stack.lookup(&ident)?;

    Ok(TypedExpr::Identifier(ident, expr.ty()))
}

fn analyze_func_declare(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    func: FuncDeclare,
) -> Result<TypedExpr, TypeError> {
    if func.is_closure {
        scope_stack.push_scope();
    } else {
        scope_stack.create_new_stack();
    }

    for param in &func.params {
        let ident = param.0.clone();
        let ty = Type::from_str(&param.1)?;
        scope_stack.insert(ident, ScopeEntry::Type(ty));
    }

    let block = analyze_block(scope_stack, *func.block)?;

    if func.is_closure {
        scope_stack.pop_scope();
    } else {
        scope_stack.restore_previous_stack();
    }

    let params: Vec<(String, Type)> = func
        .params
        .iter()
        .map(|(ident, ty)| (ident.clone(), Type::from_str(ty).unwrap()))
        .collect();

    let declared_return_type = if let Ok(return_type) = Type::from_str(&func.return_type) {
        return_type
    } else {
        return Err(TypeError {
            message: format!("Unknown return type: {}", func.return_type),
        });
    };

    let actual_return_type = block.ty();

    if declared_return_type != actual_return_type {
        // TODO: Impl Display so we don't have to use debug output here.
        return Err(TypeError {
            message: format!(
                "Return type does not match declared return type in function signature.\n\tDeclared: {:?}\n\tActual: {:?}",
                declared_return_type,
                actual_return_type,
            ),
        });
    };

    let func = TypedFunc {
        params: params.clone(),
        block: Box::new(block),
        is_closure: func.is_closure,
    };

    let mut inner_types: Vec<Type> = params.into_iter().map(|p| p.1).collect();
    inner_types.push(declared_return_type);

    Ok(TypedExpr::FuncDeclare(func, Type::Function(inner_types)))
}

fn analyze_if_else(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    cond: Expr,
    then_block: Expr,
    else_block: Expr,
) -> Result<TypedExpr, TypeError> {
    let cond = analyze_expr(scope_stack, cond)?;

    if cond.ty() != Type::Bool {
        return Err(TypeError {
            message: format!("Invalid type for if condition: {:?}", cond.ty()),
        });
    }

    let then_block = analyze_block(scope_stack, then_block)?;
    let else_block = analyze_block(scope_stack, else_block)?;

    if then_block.ty() != else_block.ty() {
        return Err(TypeError {
            message: format!(
                "Invalid types for if-else blocks: {:?} != {:?}",
                then_block.ty(),
                else_block.ty()
            ),
        });
    }

    let ty = then_block.ty();

    Ok(TypedExpr::IfElse(
        Box::new(cond),
        Box::new(then_block),
        Box::new(else_block),
        ty,
    ))
}

fn analyze_block(
    scope_stack: &mut ScopeStack<ScopeEntry>,
    block: Expr,
) -> Result<TypedExpr, TypeError> {
    scope_stack.push_scope();

    let stmts = if let Expr::Block(stmts) = block {
        analyze_stmts(scope_stack, stmts)?
    } else {
        unreachable!();
    };

    scope_stack.pop_scope();

    let ty = stmts
        .iter()
        .find(|stmt| stmt.expr.ty() != Type::Void)
        .map(|stmt| stmt.expr.ty())
        .unwrap_or(Type::Void);

    Ok(TypedExpr::Block(stmts, ty))
}
