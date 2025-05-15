use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{
    ast::typed::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    errors::runtime_error::RuntimeError,
    hashmap,
    interpreter::resolved_value::ResolvedValue,
    state::io_context::IoContext,
    types::Type,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinId {
    Int,
    PrintLine,
    ReadLine,
}

pub type BuiltinFunc<R, W> =
    fn(&mut IoContext<R, W>, Vec<ResolvedValue>) -> Result<ResolvedValue, RuntimeError>;

pub fn get_builtin_bindings() -> Vec<(String, TypedExpr)> {
    vec![
        (String::from("int"), make_int()),
        (String::from("print_line"), make_print_line()),
        (String::from("read_line"), make_read_line()),
    ]
}

pub fn get_builtins<R: Read, W: Write>() -> HashMap<BuiltinId, BuiltinFunc<R, W>> {
    hashmap! {
        BuiltinId::Int => eval_int as BuiltinFunc<R, W>,
        BuiltinId::PrintLine => eval_print_line as BuiltinFunc<R, W>,
        BuiltinId::ReadLine => eval_read_line as BuiltinFunc<R, W>
    }
}

fn make_int() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("value")],
        BuiltinId::Int,
        Type::Int,
    )));

    let func = TypedFunc {
        params: vec![(String::from("value"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::Any, Type::Int]))
}

// TODO: This is a free function right now, but we might consider implementing it as a static method
// on the Int type when that's available as an option.
fn eval_int<R: Read, W: Write>(
    _io: &mut IoContext<R, W>,
    mut args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let value = args.pop().unwrap();

    match value {
        // TODO: Parse error handling.
        ResolvedValue::String(str) => {
            Ok(ResolvedValue::Int(i32::from_str_radix(&str, 10).unwrap()))
        }
        ResolvedValue::Bool(bool) => {
            if bool {
                Ok(ResolvedValue::Int(1))
            } else {
                Ok(ResolvedValue::Int(0))
            }
        }
        _ => todo!("Implement other types."),
    }
}

fn make_print_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("message")],
        BuiltinId::PrintLine,
        Type::Void,
    )));

    let func = TypedFunc {
        params: vec![(String::from("message"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::Any, Type::Void]))
}

fn eval_print_line<R: Read, W: Write>(
    io: &mut IoContext<R, W>,
    mut args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let message = args.pop().unwrap();

    // TODO: This isn't the place to handle string coercion.
    let message = match message {
        ResolvedValue::Bool(bool) => bool.to_string(),
        ResolvedValue::Float(float) => float.to_string(),
        ResolvedValue::Function(_func) => todo!(),
        ResolvedValue::Int(int) => int.to_string(),
        ResolvedValue::String(message) => message,
        ResolvedValue::Void => String::from("Void"),
    };

    io.write_line(&message)?;

    Ok(ResolvedValue::Void)
}

fn make_read_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![],
        BuiltinId::ReadLine,
        Type::String,
    )));

    let func = TypedFunc {
        params: vec![],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::String]))
}

fn eval_read_line<R: Read, W: Write>(
    io: &mut IoContext<R, W>,
    mut _args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let line = io.read_line()?;

    Ok(ResolvedValue::String(line))
}
