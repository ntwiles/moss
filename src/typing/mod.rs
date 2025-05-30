use std::fmt::Display;

pub enum TypeBinding {
    Atomic(Type),
    Applied { arity: usize },
}

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
    Func(Vec<Type>),
    List(Box<Type>),
    Str,
    Unknown,
    UserDefined(String),
    Void,
    Applied(Box<Type>, Vec<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Bool => write!(f, "Bool"),
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Str => write!(f, "String"),
            Self::List(ty) => write!(f, "List<{}>", ty),
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
            Self::Unknown => write!(f, "Unknown"),
            Self::Void => write!(f, "Void"),
        }
    }
}
