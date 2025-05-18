use crate::{ast::typed::typed_expr::TypedExpr, typing::Type};

use super::Error;

#[derive(Debug)]
pub enum TypeError {
    AssignVoid,
    BinaryOpWrongTypes(String, Type, Type),
    IfElseBlockTypeMismatch(Type, Type),
    IfElseConditionNonBool(Type),
    DivisionZero,
    FuncWrongReturnType(Type, Type),
    InvokeNonFunc(Type),
    InvokeWrongSignature(Vec<Type>, Vec<TypedExpr>),
    UnaryOpWrongType(String, Type),
    ScopeBindingNotFound(String),
}

impl Error for TypeError {
    fn scope_binding_not_found(name: &str) -> TypeError {
        TypeError::ScopeBindingNotFound(name.to_string())
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::AssignVoid => write!(f, "Cannot assign a value of type Void."),
            TypeError::BinaryOpWrongTypes(op, a, b) => {
                write!(f, "Types {a} and {b} do not support binary operation {op}.")
            }
            TypeError::DivisionZero => write!(f, "Cannot divide by 0."),
            TypeError::FuncWrongReturnType(expected, received) => {
                write!(f, "Return type does not match declared return type in function signature.\n\tExpected: {}\n\tReceived: {}", expected, received)
            }
            TypeError::IfElseBlockTypeMismatch(expected, received) => write!(
                f,
                "Type mismatch in if-else chain.\n\tExpected: {}\n\tReceived: {}",
                expected, received
            ),
            TypeError::IfElseConditionNonBool(ty) => write!(
                f,
                "Expected conditional statement, but received expression of type {ty}"
            ),
            TypeError::InvokeNonFunc(ty) => write!(f, "Cannot invoke non-function of type {ty}"),
            TypeError::InvokeWrongSignature(param_types, args) => {
                let param_types_list = param_types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let arg_types_list = args
                    .iter()
                    .map(|t| t.ty().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(
                    f,
                    "Invoked function with the wrong signature.\n\tExpected: ({param_types_list})\n\tReceived: ({arg_types_list})"
                )
            }
            TypeError::UnaryOpWrongType(op, ty) => {
                write!(f, "Type {ty} does not support unary operation {op}.")
            }
            TypeError::ScopeBindingNotFound(ident) => {
                write!(f, "Binding \"{ident}\" not found in scope.")
            }
        }
    }
}
