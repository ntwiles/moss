use super::{ty::Type, typed_ast::typed_expr::TypedExpr};

pub enum ScopeEntry {
    TypedExpr(TypedExpr),
    Type(Type),
}

impl ScopeEntry {
    pub fn ty(&self) -> Type {
        match self {
            ScopeEntry::TypedExpr(expr) => expr.ty(),
            ScopeEntry::Type(ty) => *ty,
        }
    }

    pub fn as_expr(&self) -> &TypedExpr {
        match self {
            ScopeEntry::TypedExpr(expr) => expr,
            _ => panic!("Expected TypedExpr"),
        }
    }
}
