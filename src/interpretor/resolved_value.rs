use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ResolvedValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Display for ResolvedValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ResolvedValue::Int(int) => write!(f, "{}", int),
            ResolvedValue::Float(float) => write!(f, "{}", float),
            ResolvedValue::String(string) => write!(f, "{}", string),
            ResolvedValue::Bool(bool) => write!(f, "{}", bool),
        }
    }
}
