use std::io::{Read, Write};

use crate::{
    ast::typed::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    builtins::BuiltinFuncId,
    errors::runtime_error::RuntimeError,
    interpreter::resolved_value::ResolvedValue,
    state::io_context::IoContext,
    typing::Type,
};

// TODO: This is implemented concretely for list of strings only right now, will need to make overloads
// to support other types, or preferably add support for generics.
pub fn make_push() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("list"), String::from("item")],
        BuiltinFuncId::Push,
        Type::List(Box::new(Type::Str)),
    )));

    let func = TypedFunc {
        params: vec![
            (String::from("list"), Type::List(Box::new(Type::Str))),
            (String::from("item"), Type::Str),
        ],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(
        func,
        Type::Func(vec![
            Type::List(Box::new(Type::Str)),
            Type::Str,
            Type::List(Box::new(Type::Str)),
        ]),
    )
}

pub fn eval_push<R: Read, W: Write>(
    _io: &mut IoContext<R, W>,
    mut args: Vec<ResolvedValue>,
) -> Result<ResolvedValue, RuntimeError> {
    let item = args.pop().unwrap();
    let mut items = args.pop().unwrap().unwrap_list();

    items.push(item);

    Ok(ResolvedValue::List(items))
}
