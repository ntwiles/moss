use crate::{
    analyzer::{
        ty::Type,
        typed_ast::{typed_block::TypedBlock, typed_expr::TypedExpr, TypedFunc},
    },
    interpretor::resolved_value::ResolvedValue,
};

pub type BuiltinFunc = fn(Vec<ResolvedValue>) -> ResolvedValue;

pub fn get_builtins() -> Vec<(String, TypedExpr)> {
    return vec![
        (String::from("int"), make_int()),
        (String::from("print_line"), make_print_line()),
        (String::from("read_line"), make_read_line()),
    ];
}

fn make_int() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("value")],
        eval_int,
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
fn eval_int(mut args: Vec<ResolvedValue>) -> ResolvedValue {
    let value = args.pop().unwrap();

    match value {
        // TODO: Parse error handling.
        ResolvedValue::String(str) => ResolvedValue::Int(i32::from_str_radix(&str, 10).unwrap()),
        ResolvedValue::Bool(bool) => {
            if bool {
                ResolvedValue::Int(1)
            } else {
                ResolvedValue::Int(0)
            }
        }
        _ => todo!("Implement other types."),
    }
}

fn make_print_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![String::from("message")],
        eval_print_line,
        Type::Void,
    )));

    let func = TypedFunc {
        params: vec![(String::from("message"), Type::Any)],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::Any, Type::Void]))
}

fn eval_print_line(mut args: Vec<ResolvedValue>) -> ResolvedValue {
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

fn make_read_line() -> TypedExpr {
    let block = Box::new(TypedExpr::Block(TypedBlock::Builtin(
        vec![],
        eval_read_line,
        Type::String,
    )));

    let func = TypedFunc {
        params: vec![],
        is_closure: false,
        block,
    };

    TypedExpr::FuncDeclare(func, Type::Function(vec![Type::String]))
}

fn eval_read_line(mut _args: Vec<ResolvedValue>) -> ResolvedValue {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    if let Some('\n') = line.chars().last() {
        line.pop();
        if let Some('\r') = line.chars().last() {
            line.pop();
        }
    }

    ResolvedValue::String(line)
}
