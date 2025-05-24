use crate::typing::Type;

use super::{TypedBlock, TypedFunc, TypedFuncCall, TypedLiteral};

// TODO: For some of these, the type is always clear and maybe we don't need to store it.
#[derive(Clone, Debug)]
pub enum TypedExpr {
    // Binary operations
    Eq(Box<TypedExpr>, Box<TypedExpr>, Type),
    Gt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Lt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Gte(Box<TypedExpr>, Box<TypedExpr>, Type),
    Lte(Box<TypedExpr>, Box<TypedExpr>, Type),
    Add(Box<TypedExpr>, Box<TypedExpr>, Type),
    Sub(Box<TypedExpr>, Box<TypedExpr>, Type),
    Mult(Box<TypedExpr>, Box<TypedExpr>, Type),
    Div(Box<TypedExpr>, Box<TypedExpr>, Type),

    // Unary operations
    Negate(Box<TypedExpr>, Type),
    Declaration {
        ident: String,
        is_mutable: bool,
        expr: Box<TypedExpr>,
        ty: Type,
    }, // TODO: This doesn't need type; always void.

    // Postfix operations
    FuncCall(TypedFuncCall, Type),

    // Control flow
    If(Box<TypedExpr>, Box<TypedExpr>, Type),
    IfElse(Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>, Type),
    Block(TypedBlock),
    Loop(Box<TypedExpr>),
    Break,

    // Primaries
    Literal(TypedLiteral, Type),
    Identifier(String, Type),
    FuncDeclare(TypedFunc, Type),
    List(Vec<TypedExpr>, Type),
}

impl TypedExpr {
    pub fn ty(&self) -> Type {
        match self {
            TypedExpr::Eq(_, _, ty) => ty.clone(),
            TypedExpr::Gt(_, _, ty) => ty.clone(),
            TypedExpr::Lt(_, _, ty) => ty.clone(),
            TypedExpr::Gte(_, _, ty) => ty.clone(),
            TypedExpr::Lte(_, _, ty) => ty.clone(),
            TypedExpr::Add(_, _, ty) => ty.clone(),
            TypedExpr::Sub(_, _, ty) => ty.clone(),
            TypedExpr::Mult(_, _, ty) => ty.clone(),
            TypedExpr::Div(_, _, ty) => ty.clone(),
            TypedExpr::Literal(_, ty) => ty.clone(),
            TypedExpr::Negate(_, ty) => ty.clone(),
            TypedExpr::Declaration { ty, .. } => ty.clone(),
            TypedExpr::Identifier(_, ty) => ty.clone(),
            TypedExpr::FuncCall(_, ty) => ty.clone(),
            TypedExpr::FuncDeclare(_, ty) => ty.clone(),
            TypedExpr::If(_, _, ty) => ty.clone(),
            TypedExpr::IfElse(_, _, _, ty) => ty.clone(),
            TypedExpr::Loop(block) => block.ty(), // TODO: Is this correct? Maybe loops are always Void?
            TypedExpr::Break => Type::Void,
            TypedExpr::Block(TypedBlock::Builtin(_, _, ty)) => ty.clone(),
            TypedExpr::Block(TypedBlock::Interpreted(_, ty)) => ty.clone(),
            TypedExpr::List(_, ty) => ty.clone(),
        }
    }

    pub fn is_func_declare(&self) -> bool {
        match self {
            TypedExpr::FuncDeclare(_, _) => true,
            _ => false,
        }
    }
}
