use std::io::{Read, Write};

use crate::{
    ast::typed::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    errors::runtime_error::RuntimeError,
    interpreter::resolved_value::ResolvedValue,
    state::io_context::IoContext,
    typing::Type,
};

use crate::builtins::BuiltinFuncId;

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

pub fn make_str() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("value")],
        BuiltinFuncId::Str,
        Type::Str,
    )));

    let func = TypedFunc {
        params: vec![(String::from("value"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Func(vec![Type::Any, Type::Str]))
}

// TODO: This is a free function right now, but we might consider implementing it as a static method
// on the Int type when that's available as an option.
pub fn eval_str<R: Read, W: Write>(
    _io: &mut IoContext<R, W>,
    mut args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let value = args.pop().unwrap();

    match value {
        // TODO: Parse error handling.
        ResolvedValue::Int(i) => Ok(ResolvedValue::String(i.to_string())),
        ResolvedValue::Bool(bool) => {
            if bool {
                Ok(ResolvedValue::String("True".to_string()))
            } else {
                Ok(ResolvedValue::String("False".to_string()))
            }
        }
        _ => todo!("Implement other types."),
    }
}
