use crate::errors::{type_error::TypeError, Error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Any, // TODO: Temporary for string coercion in print. Do not use, and remove when generics are implemented.
    Int,
    Float,
    String,
    Bool,
    Void,
    Function(Vec<Type>),
}

impl Type {
    pub fn from_str(s: &str) -> Result<Type, TypeError> {
        match s {
            "Int" => Ok(Type::Int),
            "Float" => Ok(Type::Float),
            "String" => Ok(Type::String),
            "Bool" => Ok(Type::Bool),
            "Void" => Ok(Type::Void),
            "Function" => Ok(Type::Function(vec![])), // TODO: Whatever from_str is used for, it probably needs to support the full function type.
            _ => Err(TypeError::new(format!("Unknown type: {}", s))),
        }
    }
}
