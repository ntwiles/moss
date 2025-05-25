use std::fmt::Formatter;

use crate::{
    ast::{typed::typed_expr::TypedExpr, Span},
    typing::Type,
};

use super::Error;

#[derive(Debug)]
pub enum TypeError {
    AmbiguousListType,
    AssignWrongType(Type, Type),
    AssignImmutable(String),
    AssignVoid,
    BinaryOpWrongTypes(String, Type, Type),
    DivisionZero,
    ExpectedTypeReceivedList(Type),
    FuncWrongReturnType(Type, Type, Span),
    IfElseBlockTypeMismatch(Type, Type),
    IfElseConditionNonBool(Type),
    InvokeNonFunc(Type),
    InvokeWrongSignature(Vec<Type>, Vec<TypedExpr>, Span),
    UnaryOpWrongType(String, Type),
    ScopeBindingAlreadyExists(String),
    ScopeBindingNotFound(String),
    AppliedTypeWrongNumberArgs(String, usize, usize),
}

impl TypeError {
    pub fn display(self, file_name: String, source: String) -> TypeErrorDisplay {
        TypeErrorDisplay {
            error: self,
            file_name,
            source,
        }
    }
}

impl Error for TypeError {
    fn scope_binding_already_exists(ident: &str) -> Self {
        TypeError::ScopeBindingAlreadyExists(ident.to_string())
    }
    fn scope_binding_not_found(name: &str) -> Self {
        TypeError::ScopeBindingNotFound(name.to_string())
    }
}

pub struct TypeErrorDisplay {
    error: TypeError,
    file_name: String,
    source: String,
}

impl std::fmt::Display for TypeErrorDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.error {
            TypeError::AmbiguousListType => write!(f, "Cannot resolve list element type."),
            TypeError::AppliedTypeWrongNumberArgs(ty, expected, received) => {
                // 1. Header
                writeln!(f, "Received wrong number of type arguments for type {ty}. ")?;

                // 2. Dianostic Detail
                writeln!(f, "Expected: {expected}\nReceived: {received}")
            }

            TypeError::AssignImmutable(ident) => {
                write!(f, "Cannot re-assign immutable binding \"{ident}\".")
            }
            TypeError::AssignWrongType(expected, received) => write!(
                f,
                "Cannot assign a value of type {received} where type {expected} is expected"
            ),
            TypeError::AssignVoid => write!(f, "Cannot assign a value of type Void."),
            TypeError::BinaryOpWrongTypes(op, a, b) => {
                write!(f, "Types {a} and {b} do not support binary operation {op}.")
            }
            TypeError::DivisionZero => write!(f, "Cannot divide by 0."),
            TypeError::ExpectedTypeReceivedList(expected) => write!(
                f,
                "Expected a value of type {expected}, but received a list of unknown type."
            ),
            TypeError::FuncWrongReturnType(expected, received, span) => {
                // 1. Header
                writeln!(
                    f,
                    "Return type does not match declared return type in function signature."
                )?;

                // 2. Location Frame
                write_location_frame(f, &self.file_name, &self.source, span.start, span.end)?;

                // 3. Diagnostic Detail
                writeln!(f, "Expected: {}\nReceived: {}", expected, received)
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
            TypeError::InvokeWrongSignature(param_types, args, span) => {
                // 1. Header
                writeln!(f, "Invoked function with the wrong signature.")?;

                // 2. Location Frame
                write_location_frame(f, &self.file_name, &self.source, span.start, span.end)?;

                // 3. Diagnostic Detail
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

                writeln!(
                    f,
                    "Expected: ({param_types_list})\nReceived: ({arg_types_list})"
                )
            }
            TypeError::UnaryOpWrongType(op, ty) => {
                write!(f, "Type {ty} does not support unary operation {op}.")
            }
            TypeError::ScopeBindingAlreadyExists(ident) => {
                write!(f, "Binding \"{ident}\" already exists in local scope.")
            }
            TypeError::ScopeBindingNotFound(ident) => {
                write!(f, "Binding \"{ident}\" not found in scope.")
            }
        }
    }
}

fn write_location_frame(
    f: &mut Formatter,
    file_name: &str,
    source: &str,
    start: usize,
    end: usize,
) -> std::fmt::Result {
    let line_num = get_line_number(source, start);
    let file_label = format!("{}:{line_num}", file_name);
    let snippet = &source[start..end];

    writeln!(f, "{:-^width$}", file_label, width = 80)?;
    writeln!(f, "{snippet}")?;
    writeln!(f, "{:-<width$}", "", width = 80)
}

fn get_line_number(source: &str, byte_offset: usize) -> usize {
    source[..byte_offset].lines().count()
}
