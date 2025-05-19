pub mod analyzer;
mod ast;
pub mod builtins;
mod errors;
pub mod interpreter;
mod scopes;
mod state;
pub mod test_util;
mod typing;
mod util;

lalrpop_mod!(pub grammar);

use builtins::{get_builtin_func_bindings, get_builtin_funcs, get_builtin_type_bindings};
use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use scopes::scope_stack::ScopeStack;
use state::{exec_context::ExecContext, io_context::IoContext};
use std::{
    env, fs,
    io::{self, BufReader, BufWriter},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let code = fs::read_to_string(path);

    let code = match code {
        Ok(code) => code,
        Err(err) => {
            println!("Error loading file: {}", err);
            return;
        }
    };

    let parsed = ProgramParser::new().parse(&code);

    if let Err(error) = parsed {
        println!("Parse Error: {}", error);
        return;
    }

    let analyzed = analyzer::analyze_program(
        parsed.unwrap(),
        get_builtin_func_bindings(),
        get_builtin_type_bindings(),
    );

    if let Err(error) = analyzed {
        println!("Type Error: {}", error.display(path.clone(), code));
        return;
    }

    let run_result = interpreter::interpret_program(
        analyzed.unwrap(),
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
    );

    if let Err(error) = run_result {
        println!("Runtime Error: {}", error.message);
        return;
    }
}
