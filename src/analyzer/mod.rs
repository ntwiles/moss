mod binary_ops;
mod scope_entry;

use crate::ast::typed::typed_block::TypedBlock;
use crate::ast::typed::typed_expr::TypedExpr;
use crate::ast::typed::{TypedFunc, TypedFuncCall, TypedLiteral, TypedStmt};
use crate::ast::untyped::{Expr, FuncCall, FuncDeclare, Literal, Stmt};
use crate::ast::Span;
use crate::errors::type_error::TypeError;
use crate::scopes::scope::Scope;
use crate::scopes::scope_stack::ScopeStack;
use crate::typing::{ProtoType, Type, TypeBinding};

use binary_ops::analyze_binary_op;
use scope_entry::AnalyzerScopeEntry;

pub fn analyze_program(
    stmts: Expr,
    builtin_funcs: Vec<(String, TypedExpr)>,
    builtin_types: Vec<(String, TypeBinding)>,
) -> Result<TypedExpr, TypeError> {
    let mut value_scope_stack = ScopeStack::<AnalyzerScopeEntry>::new();

    for (ident, binding) in builtin_funcs {
        value_scope_stack.insert(ident, false, binding.ty())?;
    }

    let mut type_scope = Scope::new();

    for (ident, binding) in builtin_types {
        type_scope.insert(ident, binding);
    }

    analyze_block(&mut value_scope_stack, &mut type_scope, stmts)
}

fn analyze_stmts(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    stmts: Vec<Stmt>,
) -> Result<Vec<TypedStmt>, TypeError> {
    stmts
        .into_iter()
        .map(|stmt| analyze_stmt(value_scope_stack, type_scope, stmt))
        .collect()
}

fn analyze_stmt(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    stmt: Stmt,
) -> Result<TypedStmt, TypeError> {
    let expr = analyze_expr(value_scope_stack, type_scope, &None, stmt.expr)?;

    Ok(TypedStmt { expr })
}

fn analyze_expr(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    type_hint: &Option<Type>,
    expr: Expr,
) -> Result<TypedExpr, TypeError> {
    use Expr::*;

    match expr {
        Block(stmts, span) => {
            analyze_block(value_scope_stack, type_scope, Expr::Block(stmts, span))
        }
        Literal(literal) => analyze_literal(literal),
        Identifier(ident) => analyze_identifier(value_scope_stack, ident),

        // TODO: `analyze_binary_op` is an experiment, let's try this out for a while and see how it
        // goes. I have concerns with this approach because it moves the non-exhaustive match error
        // to runtime.
        // Binary ops
        Eq(..) | Gt(..) | Lt(..) | Gte(..) | Lte(..) | Add(..) | Sub(..) | Mult(..) | Div(..)
        | Modulo(..) => analyze_binary_op(value_scope_stack, type_scope, expr),

        // Unary ops
        Negate(inner) => analyze_negate(value_scope_stack, type_scope, *inner),
        Assignment { ident, expr } => {
            analyze_assignment(value_scope_stack, type_scope, ident, *expr)
        }
        Declaration {
            ident,
            type_annotation,
            expr,
            is_mutable,
        } => analyze_declaration(
            value_scope_stack,
            type_scope,
            ident,
            type_annotation,
            is_mutable,
            *expr,
        ),

        FuncDeclare(func) => analyze_func_declare(value_scope_stack, type_scope, func),
        FuncCall(call, span) => analyze_func_call(value_scope_stack, type_scope, call, span),

        // Control flow
        If(expr, then) => analyze_if(value_scope_stack, type_scope, *expr, *then),
        IfElse(expr, then, els) => {
            analyze_if_else(value_scope_stack, type_scope, *expr, *then, *els)
        }
        Loop(block) => analyze_loop(value_scope_stack, type_scope, *block),
        Break => analyze_break(value_scope_stack),
        List(values) => analyze_list(value_scope_stack, type_scope, type_hint, values),
    }
}

// Unary operations

fn analyze_negate(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    inner: Expr,
) -> Result<TypedExpr, TypeError> {
    let inner = analyze_expr(value_scope_stack, type_scope, &None, inner)?;

    if inner.ty() != Type::Int && inner.ty() != Type::Float {
        return Err(TypeError::UnaryOpWrongType("-".to_string(), inner.ty()));
    }

    let ty = inner.ty();
    Ok(TypedExpr::Negate(Box::new(inner), ty))
}

fn analyze_assignment(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    ident: String,
    expr: Expr,
) -> Result<TypedExpr, TypeError> {
    if !value_scope_stack.lookup(&ident)?.is_mutable {
        return Err(TypeError::AssignImmutable(ident));
    }

    let expr = analyze_expr(value_scope_stack, type_scope, &None, expr)?;

    Ok(TypedExpr::Assignment {
        ident,
        expr: Box::new(expr),
    })
}

fn analyze_declaration(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    ident: String,
    type_annotation: Option<ProtoType>,
    is_mutable: bool,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    // TODO: There's a lot of code duplication between these two. They're separate now because in the
    // case of type checking function declarations, the function has to be bound to scope prior to
    // analyzing the funciton body, to allow for recursion. In all other cases, the value expression
    // is analyzed before binding the identifier.
    if value.is_func_declare() {
        analyze_func_declaration(value_scope_stack, type_scope, ident, is_mutable, value)
    } else {
        analyze_non_func_declaration(
            value_scope_stack,
            type_scope,
            ident,
            is_mutable,
            type_annotation,
            value,
        )
    }
}

fn analyze_non_func_declaration(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    ident: String,
    is_mutable: bool,
    type_annotation: Option<ProtoType>,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    let type_annotation = type_annotation
        .map(|a| analyze_proto_type(type_scope, a))
        .transpose()?;

    let value = analyze_expr(value_scope_stack, type_scope, &type_annotation, value)?;
    let value_type = value.ty();

    if value_type == Type::Void {
        return Err(TypeError::AssignVoid);
    }

    if let Some(annotation) = type_annotation {
        if value_type != annotation {
            return Err(TypeError::AssignWrongType(annotation, value_type));
        }
    }

    value_scope_stack.insert(ident.clone(), is_mutable, value_type)?;

    Ok(TypedExpr::Declaration {
        ident,
        is_mutable,
        expr: Box::new(value),
        ty: Type::Void,
    })
}

fn analyze_func_declaration(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    ident: String,
    is_mutable: bool,
    value: Expr,
) -> Result<TypedExpr, TypeError> {
    let func = value.as_func_declare();

    let return_type = analyze_proto_type(type_scope, *func.return_type.clone())?;
    let mut type_args: Vec<_> = func
        .params
        .iter()
        .map(|(_, proto)| analyze_proto_type(type_scope, proto.clone()))
        .collect::<Result<_, _>>()?;

    type_args.push(return_type);

    value_scope_stack.insert(ident.clone(), is_mutable, Type::Func(type_args))?;

    let value = analyze_expr(value_scope_stack, type_scope, &None, value)?;

    if value.ty() == Type::Void {
        return Err(TypeError::AssignVoid);
    }

    Ok(TypedExpr::Declaration {
        ident,
        is_mutable,
        expr: Box::new(value),
        ty: Type::Void,
    })
}

// Postfix operations

fn analyze_func_call(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    call: FuncCall,
    span: Span,
) -> Result<TypedExpr, TypeError> {
    let callee = analyze_expr(value_scope_stack, type_scope, &None, *call.func)?;

    let args = call
        .args
        .into_iter()
        .map(|arg| analyze_expr(value_scope_stack, type_scope, &None, arg))
        .collect::<Result<Vec<_>, _>>()?;

    if let Type::Func(inner_types) = callee.ty() {
        let mut inner_types = inner_types.clone();

        let return_type = inner_types.pop().unwrap();
        let param_types = inner_types;

        if param_types.len() != args.len() {
            return Err(TypeError::InvokeWrongSignature(param_types, args, span));
        }

        for (param_type, arg) in param_types.clone().into_iter().zip(args.clone()) {
            if arg.ty() != param_type && param_type != Type::Any {
                return Err(TypeError::InvokeWrongSignature(param_types, args, span));
            }
        }

        let func_call = TypedFuncCall {
            func_expr: Box::new(callee),
            args,
        };

        Ok(TypedExpr::FuncCall(func_call, return_type))
    } else {
        return Err(TypeError::InvokeNonFunc(callee.ty()));
    }
}

// Primaries

fn analyze_literal(literal: Literal) -> Result<TypedExpr, TypeError> {
    Ok(match literal {
        Literal::Int(i) => TypedExpr::Literal(TypedLiteral::Int(i), Type::Int),
        Literal::Float(f) => TypedExpr::Literal(TypedLiteral::Float(f), Type::Float),
        Literal::String(s) => TypedExpr::Literal(TypedLiteral::String(s), Type::Str),
        Literal::Bool(b) => TypedExpr::Literal(TypedLiteral::Bool(b), Type::Bool),
    })
}

// TODO: Should this resolve the identifier from the scope and error of it's just a Type?
fn analyze_identifier(
    scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    ident: String,
) -> Result<TypedExpr, TypeError> {
    let ty = scope_stack.lookup(&ident)?.value.clone();
    Ok(TypedExpr::Identifier(ident, ty.clone()))
}

fn analyze_func_declare(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    func: FuncDeclare,
) -> Result<TypedExpr, TypeError> {
    if func.is_closure {
        value_scope_stack.push_scope();
    } else {
        value_scope_stack.create_new_stack();
    }

    for param in &func.params {
        let ident = param.0.clone();
        let ty = analyze_proto_type(type_scope, param.1.clone())?;
        value_scope_stack.insert(ident, false, ty)?;
    }

    let block = *func.block;

    let span = if let Expr::Block(_, span) = &block {
        span.clone()
    } else {
        todo!();
    };

    let block = analyze_block(value_scope_stack, type_scope, block)?;

    if func.is_closure {
        value_scope_stack.pop_scope();
    } else {
        value_scope_stack.restore_previous_stack();
    }

    let params: Vec<(String, Type)> = func
        .params
        .iter()
        .map(|(ident, ty)| {
            let ty = analyze_proto_type(type_scope, ty.clone())?;
            Ok((ident.clone(), ty))
        })
        .collect::<Result<_, TypeError>>()?;

    let declared_return_type = analyze_proto_type(type_scope, *func.return_type)?;
    let actual_return_type = block.ty();

    if declared_return_type != actual_return_type {
        return Err(TypeError::FuncWrongReturnType(
            declared_return_type,
            actual_return_type,
            span,
        ));
    };

    let func = TypedFunc {
        params: params.clone(),
        block: Box::new(block),
        is_closure: func.is_closure,
    };

    let mut inner_types: Vec<Type> = params.into_iter().map(|p| p.1).collect();
    inner_types.push(declared_return_type);

    Ok(TypedExpr::FuncDeclare(func, Type::Func(inner_types)))
}

fn analyze_if(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    cond: Expr,
    then_block: Expr,
) -> Result<TypedExpr, TypeError> {
    let cond = analyze_expr(value_scope_stack, type_scope, &None, cond)?;

    if cond.ty() != Type::Bool {
        return Err(TypeError::IfElseConditionNonBool(cond.ty()));
    }

    let then_block = analyze_block(value_scope_stack, type_scope, then_block)?;

    let ty = then_block.ty();

    Ok(TypedExpr::If(Box::new(cond), Box::new(then_block), ty))
}

fn analyze_if_else(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    cond: Expr,
    then_block: Expr,
    else_expr: Expr,
) -> Result<TypedExpr, TypeError> {
    let cond = analyze_expr(value_scope_stack, type_scope, &None, cond)?;

    if cond.ty() != Type::Bool {
        return Err(TypeError::IfElseConditionNonBool(cond.ty()));
    }

    let then_block = analyze_block(value_scope_stack, type_scope, then_block)?;
    let else_expr = analyze_expr(value_scope_stack, type_scope, &None, else_expr)?;

    if then_block.ty() != else_expr.ty() {
        return Err(TypeError::IfElseBlockTypeMismatch(
            then_block.ty(),
            else_expr.ty(),
        ));
    }

    let ty = then_block.ty();

    Ok(TypedExpr::IfElse(
        Box::new(cond),
        Box::new(then_block),
        Box::new(else_expr),
        ty,
    ))
}

fn analyze_loop(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    block: Expr,
) -> Result<TypedExpr, TypeError> {
    // TODO: Currently just using this as a wrapper to analyze the block, there may be more we can
    // do here later though.
    let block = analyze_block(value_scope_stack, type_scope, block.clone())?;
    Ok(TypedExpr::Loop(Box::new(block)))
}

fn analyze_break(
    _scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
) -> Result<TypedExpr, TypeError> {
    // TODO: Check if break is used in the right context. This might be a new type of error like
    // ContextError or it might make sense to treat as a TypeError.

    Ok(TypedExpr::Break)
}

fn analyze_block(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    block: Expr,
) -> Result<TypedExpr, TypeError> {
    value_scope_stack.push_scope();

    let stmts = if let Expr::Block(stmts, _) = block {
        analyze_stmts(value_scope_stack, type_scope, stmts)?
    } else {
        unreachable!();
    };

    value_scope_stack.pop_scope();

    let ty = stmts
        .iter()
        .find(|stmt| stmt.expr.ty() != Type::Void)
        .map(|stmt| stmt.expr.ty())
        .unwrap_or(Type::Void);

    Ok(TypedExpr::Block(TypedBlock::Interpreted(stmts, ty)))
}

fn analyze_proto_type(
    type_scope: &mut Scope<TypeBinding>,
    proto: ProtoType,
) -> Result<Type, TypeError> {
    match proto {
        ProtoType::Atomic(ident) => {
            let binding = type_scope
                .get(&ident)
                .ok_or(TypeError::ScopeBindingNotFound(ident))?;

            if let TypeBinding::Atomic(ty) = binding {
                Ok(ty.clone())
            } else {
                todo!();
            }
        }
        ProtoType::Applied(ident, inners) => {
            let binding = type_scope
                .get(&ident)
                .ok_or(TypeError::ScopeBindingNotFound(ident.clone()))?;

            if let TypeBinding::Applied { arity } = binding {
                if inners.len() != *arity {
                    return Err(TypeError::AppliedTypeWrongNumberArgs(
                        ident,
                        *arity,
                        inners.len(),
                    ));
                }

                // This is probably best done with type constructors bound to the scope instead of
                // being hardcoded here.
                match ident.as_str() {
                    "Func" => Ok(Type::Func(
                        inners
                            .iter()
                            .map(|proto| analyze_proto_type(type_scope, proto.clone()))
                            .collect::<Result<_, _>>()?,
                    )),
                    "List" => {
                        if inners.len() != 1 {
                            todo!("Correct error handling.");
                        }

                        let inner =
                            analyze_proto_type(type_scope, inners.first().unwrap().clone())?;

                        Ok(Type::List(Box::new(inner)))
                    }
                    _ => todo!(),
                }
            } else {
                todo!()
            }
        }
    }
}

fn analyze_list(
    value_scope_stack: &mut ScopeStack<AnalyzerScopeEntry>,
    type_scope: &mut Scope<TypeBinding>,
    type_hint: &Option<Type>,
    values: Vec<Expr>,
) -> Result<TypedExpr, TypeError> {
    let element_hint = match type_hint {
        Some(Type::List(inner)) => Some(inner.as_ref().clone()),
        Some(type_hint) => return Err(TypeError::ExpectedTypeReceivedList(type_hint.clone())),
        None => None,
    };

    let mut typed_values = Vec::with_capacity(values.len());

    for v in values {
        typed_values.push(analyze_expr(value_scope_stack, type_scope, &None, v)?);
    }

    let list_type = typed_values.first().map(|t| t.ty());

    let list_type = if let Some(list_type) = list_type {
        list_type
    } else {
        if let Some(element_hint) = element_hint {
            Ok(element_hint)?
        } else {
            Err(TypeError::AmbiguousListType)?
        }
    };

    Ok(TypedExpr::List(
        typed_values,
        Type::List(Box::new(list_type)),
    ))
}
