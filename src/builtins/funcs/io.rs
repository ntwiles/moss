use std::io::{Read, Write};

use crate::{
    ast::typed::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    builtins::BuiltinFuncId,
    errors::runtime_error::RuntimeError,
    interpreter::resolved_value::ResolvedValue,
    state::io_context::IoContext,
    typing::Type,
};

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

    // TODO: This isn't the place to handle string coercion. Probably it should accept only values
    // which are known to be able to be converted to a string, though some type of polymorphism.
    // TODO: Even though this function is typed to support any input, it will error at runtime when
    // lists are passed. Will likely be resolved by the above fix.
    let message = match message {
        ResolvedValue::Bool(bool) => bool.to_string(),
        ResolvedValue::Float(float) => float.to_string(),
        ResolvedValue::Func(func) => func.to_string(),
        ResolvedValue::Int(int) => int.to_string(),
        ResolvedValue::List(items) => {
            let inner = items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");

            format!("[{inner}]")
        }
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
        Type::Str,
    )));

    let func = TypedFunc {
        params: vec![],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Func(vec![Type::Str]))
}

pub fn eval_read_line<R: Read, W: Write>(
    io: &mut IoContext<R, W>,
    mut _args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let line = io.read_line()?;

    Ok(ResolvedValue::String(line))
}
