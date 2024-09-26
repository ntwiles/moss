use crate::errors::{type_error::TypeError, Error};

#[derive(Copy, Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
    Function,
}

impl Type {
    pub fn from_str(s: &str) -> Result<Type, TypeError> {
        match s {
            "Int" => Ok(Type::Int),
            "Float" => Ok(Type::Float),
            "String" => Ok(Type::String),
            "Bool" => Ok(Type::Bool),
            "Void" => Ok(Type::Void),
            "Function" => Ok(Type::Function),
            _ => Err(TypeError::new(format!("Unknown type: {}", s))),
        }
    }
}
