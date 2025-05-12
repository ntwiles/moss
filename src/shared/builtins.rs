use crate::{
    analyzer::{
        ty::Type,
        typed_ast::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    },
    interpretor::resolved_value::ResolvedValue,
};

pub type BuiltinFunc = fn(Vec<ResolvedValue>) -> ResolvedValue;

pub fn get_builtins() -> Vec<(String, TypedExpr)> {
    return vec![(String::from("print"), make_print())];
}

fn make_print() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        eval_print,
        Type::Void,
    )));

    let func = TypedFunc {
        params: vec![(String::from("message"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::Any, Type::Void]))
}

fn eval_print(mut args: Vec<ResolvedValue>) -> ResolvedValue {
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

    println!("{}", message);

    ResolvedValue::Void
}
