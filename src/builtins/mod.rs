mod funcs;

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use funcs::{
    eval_int, eval_print_line, eval_read_line, eval_str, make_int, make_print_line, make_read_line,
    make_str,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinFuncId {
    Int,
    PrintLine,
    ReadLine,
    Str,
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
        (String::from("str"), make_str()),
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
        BuiltinFuncId::ReadLine => eval_read_line as BuiltinFunc<R, W>,
        BuiltinFuncId::Str => eval_str as BuiltinFunc<R, W>
    }
}
