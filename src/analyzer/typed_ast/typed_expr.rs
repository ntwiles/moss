use crate::analyzer::ty::Type;

use super::{TypedFunc, TypedFuncCall, TypedLiteral, TypedStmt};

// TODO: For some of these, the type is always clear and maybe we don't need to store it.
#[derive(Clone, Debug)]
pub enum TypedExpr {
    // Binary operations
    Eq(Box<TypedExpr>, Box<TypedExpr>, Type),
    Gt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Lt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Add(Box<TypedExpr>, Box<TypedExpr>, Type),
    Sub(Box<TypedExpr>, Box<TypedExpr>, Type),
    Mult(Box<TypedExpr>, Box<TypedExpr>, Type),
    Div(Box<TypedExpr>, Box<TypedExpr>, Type),

    // Unary operations
    Negate(Box<TypedExpr>, Type),
    Assign(String, Box<TypedExpr>, Type),

    // Postfix operations
    FuncCall(TypedFuncCall, Type),

    // Control flow
    IfElse(Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>, Type),
    Block(Vec<TypedStmt>, Type),

    // Primaries
    Literal(TypedLiteral, Type),
    Identifier(String, Type),
    FuncDeclare(TypedFunc, Type),
}

impl TypedExpr {
    pub fn ty(&self) -> Type {
        match self {
            TypedExpr::Eq(_, _, ty) => ty.clone(),
            TypedExpr::Gt(_, _, ty) => ty.clone(),
            TypedExpr::Lt(_, _, ty) => ty.clone(),
            TypedExpr::Add(_, _, ty) => ty.clone(),
            TypedExpr::Sub(_, _, ty) => ty.clone(),
            TypedExpr::Mult(_, _, ty) => ty.clone(),
            TypedExpr::Div(_, _, ty) => ty.clone(),
            TypedExpr::Literal(_, ty) => ty.clone(),
            TypedExpr::Negate(_, ty) => ty.clone(),
            TypedExpr::Assign(_, _, ty) => ty.clone(),
            TypedExpr::Identifier(_, ty) => ty.clone(),
            TypedExpr::FuncCall(_, ty) => ty.clone(),
            TypedExpr::FuncDeclare(_, ty) => ty.clone(),
            TypedExpr::IfElse(_, _, _, ty) => ty.clone(),
            TypedExpr::Block(_, ty) => ty.clone(),
        }
    }

    pub fn is_func_declare(&self) -> bool {
        match self {
            TypedExpr::FuncDeclare(_, _) => true,
            _ => false,
        }
    }
}
