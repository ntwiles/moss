use std::fmt::{Display, Formatter, Result};

use crate::ast::typed::TypedFunc;

#[derive(Clone, Debug)]
pub enum ResolvedValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Void,
    Func(TypedFunc),
}

impl ResolvedValue {
    pub fn unwrap_int(&self) -> i32 {
        match self {
            ResolvedValue::Int(int) => *int,
            _ => panic!(),
        }
    }

    pub fn unwrap_float(&self) -> f64 {
        match self {
            ResolvedValue::Float(float) => *float,
            _ => panic!(),
        }
    }

    pub fn unwrap_string(&self) -> String {
        match self {
            ResolvedValue::String(string) => string.clone(),
            _ => panic!(),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            ResolvedValue::Bool(bool) => *bool,
            _ => panic!(),
        }
    }

    pub fn unwrap_void(&self) {
        match self {
            ResolvedValue::Void => (),
            _ => panic!(),
        }
    }
}

impl Display for ResolvedValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ResolvedValue::Int(int) => write!(f, "{}", int),
            ResolvedValue::Float(float) => write!(f, "{:.1}", float),
            ResolvedValue::String(string) => write!(f, "{}", string),
            ResolvedValue::Bool(bool) => write!(f, "{}", bool),
            ResolvedValue::Void => write!(f, "Void"),
            ResolvedValue::Func(func) => write!(f, "{func}"),
        }
    }
}
