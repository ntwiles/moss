use std::io::{self, BufReader, BufWriter};

use crate::{
    analyzer,
    ast::{typed::typed_expr::TypedExpr, untyped::Expr},
    builtins::{get_builtin_func_bindings, get_builtin_funcs, get_builtin_type_bindings},
    errors::{runtime_error::RuntimeError, type_error::TypeError},
    interpreter::{self, resolved_value::ResolvedValue},
    scopes::scope_stack::ScopeStack,
    state::{exec_context::ExecContext, io_context::IoContext},
};

pub fn analyze_program(program: Expr) -> Result<TypedExpr, TypeError> {
    analyzer::analyze_program(
        program,
        get_builtin_func_bindings(),
        get_builtin_type_bindings(),
    )
}

pub fn exec_program(program: TypedExpr) -> Result<ResolvedValue, RuntimeError> {
    interpreter::interpret_program(
        program,
        ExecContext {
            control_stack: Vec::new(),
            value_stack: Vec::new(),
            scope_stack: ScopeStack::new(),
        },
        IoContext {
            reader: BufReader::new(io::stdin().lock()),
            writer: BufWriter::new(io::stdout().lock()),
        },
        get_builtin_func_bindings(),
        get_builtin_funcs(),
    )
}
