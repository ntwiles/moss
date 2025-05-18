mod funcs;

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use funcs::{eval_int, eval_print_line, eval_read_line, make_int, make_print_line, make_read_line};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinFuncId {
    Int,
    PrintLine,
    ReadLine,
}

pub type BuiltinFunc<R, W> =
    fn(&mut IoContext<R, W>, Vec<ResolvedValue>) -> Result<ResolvedValue, RuntimeError>;

use crate::{
    ast::typed::typed_expr::TypedExpr, errors::runtime_error::RuntimeError, hashmap,
    interpreter::resolved_value::ResolvedValue, state::io_context::IoContext, typing::Type,
};

pub fn get_builtin_func_bindings() -> Vec<(String, TypedExpr)> {
    vec![
        (String::from("int"), make_int()),
        (String::from("print_line"), make_print_line()),
        (String::from("read_line"), make_read_line()),
    ]
}

pub fn get_builtin_type_bindings() -> Vec<(String, Type)> {
    vec![
        (String::from("Int"), Type::Int),
        (String::from("Bool"), Type::Bool),
        (String::from("Float"), Type::Float),
        (String::from("String"), Type::String),
        (String::from("Void"), Type::Void),
        (String::from("Bool"), Type::Bool),
    ]
}

pub fn get_builtin_funcs<R: Read, W: Write>() -> HashMap<BuiltinFuncId, BuiltinFunc<R, W>> {
    hashmap! {
        BuiltinFuncId::Int => eval_int as BuiltinFunc<R, W>,
        BuiltinFuncId::PrintLine => eval_print_line as BuiltinFunc<R, W>,
        BuiltinFuncId::ReadLine => eval_read_line as BuiltinFunc<R, W>
    }
}
