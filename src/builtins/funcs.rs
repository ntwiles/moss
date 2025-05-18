use std::io::{Read, Write};

use crate::{
    ast::typed::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    errors::runtime_error::RuntimeError,
    interpreter::resolved_value::ResolvedValue,
    state::io_context::IoContext,
    typing::Type,
};

use super::BuiltinFuncId;

pub fn make_int() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("value")],
        BuiltinFuncId::Int,
        Type::Int,
    )));

    let func = TypedFunc {
        params: vec![(String::from("value"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Func(vec![Type::Any, Type::Int]))
}

// TODO: This is a free function right now, but we might consider implementing it as a static method
// on the Int type when that's available as an option.
pub fn eval_int<R: Read, W: Write>(
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

pub fn make_print_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("message")],
        BuiltinFuncId::PrintLine,
        Type::Void,
    )));

    let func = TypedFunc {
        params: vec![(String::from("message"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Func(vec![Type::Any, Type::Void]))
}

pub fn eval_print_line<R: Read, W: Write>(
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

pub fn make_read_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![],
        BuiltinFuncId::ReadLine,
        Type::String,
    )));

    let func = TypedFunc {
        params: vec![],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Func(vec![Type::String]))
}

pub fn eval_read_line<R: Read, W: Write>(
    io: &mut IoContext<R, W>,
    mut _args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let line = io.read_line()?;

    Ok(ResolvedValue::String(line))
}
