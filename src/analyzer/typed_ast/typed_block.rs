use crate::{analyzer::ty::Type, shared::builtins::BuiltinFunc};

use super::TypedStmt;

#[derive(Clone, Debug)]
pub enum TypedBlock {
    Interpreted(Vec<TypedStmt>, Type),
    Builtin(Vec<String>, BuiltinFunc, Type),
}

impl TypedBlock {
    pub fn ty(&self) -> Type {
        match self {
            TypedBlock::Builtin(_, _, ty) => ty.clone(),
            TypedBlock::Interpreted(_, ty) => ty.clone(),
        }
    }
}
