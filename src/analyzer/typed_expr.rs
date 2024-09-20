use super::{ty::Type, TypedLiteral};

#[derive(Debug)]
pub enum TypedExpr {
    Eq(Box<TypedExpr>, Box<TypedExpr>, Type),
    Gt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Lt(Box<TypedExpr>, Box<TypedExpr>, Type),
    Add(Box<TypedExpr>, Box<TypedExpr>, Type),
    Sub(Box<TypedExpr>, Box<TypedExpr>, Type),
    Mult(Box<TypedExpr>, Box<TypedExpr>, Type),
    Div(Box<TypedExpr>, Box<TypedExpr>, Type),
    Literal(TypedLiteral, Type),
    Negate(Box<TypedExpr>, Type),
}

impl TypedExpr {
    pub fn ty(&self) -> Type {
        match self {
            TypedExpr::Eq(_, _, ty) => *ty,
            TypedExpr::Gt(_, _, ty) => *ty,
            TypedExpr::Lt(_, _, ty) => *ty,
            TypedExpr::Add(_, _, ty) => *ty,
            TypedExpr::Sub(_, _, ty) => *ty,
            TypedExpr::Mult(_, _, ty) => *ty,
            TypedExpr::Div(_, _, ty) => *ty,
            TypedExpr::Literal(_, ty) => *ty,
            TypedExpr::Negate(_, ty) => *ty,
        }
    }
}
