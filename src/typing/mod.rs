use std::fmt::Display;

use crate::errors::{type_error::TypeError, Error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProtoType {
    Atomic(String),
    Applied(String, Vec<ProtoType>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Any, // TODO: Temporary for string coercion in print. Do not use, and remove when generics are implemented.
    Bool,
    Int,
    Float,
    String,
    Func(Vec<Type>),
    UserDefined(String),
    Applied(Box<Type>, Vec<Type>),
    Void,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Bool => write!(f, "Bool"),
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::String => write!(f, "String"),
            Self::Func(params) => {
                let inner = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "Func<{}>", inner)
            }
            Self::UserDefined(ident) => write!(f, "{ident}"),
            Self::Applied(outer, inner) => {
                let inner = inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{outer}<{inner}>")
            }
            Self::Void => write!(f, "Void"),
        }
    }
}

impl Type {
    // TODO: Kill this.
    pub fn from_str(s: &str) -> Result<Type, TypeError> {
        match s {
            "Int" => Ok(Type::Int),
            "Float" => Ok(Type::Float),
            "String" => Ok(Type::String),
            "Bool" => Ok(Type::Bool),
            "Void" => Ok(Type::Void),
            "Func" => Ok(Type::Func(vec![])), // TODO: Whatever from_str is used for, it probably needs to support the full function type.
            _ => Err(TypeError::new(format!("Unknown type: {}", s))),
        }
    }
}
