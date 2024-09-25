pub mod analyzer;
mod ast;
mod errors;
pub mod interpretor;
mod scope_stack;
lalrpop_mod!(pub grammar);

use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use std::{env, fs};

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

    let analyzed = analyzer::analyze_program(parsed.unwrap());

    if let Err(error) = analyzed {
        println!("Type Error: {}", error.message);
        return;
    }

    let run_result = interpretor::interpret_lines(analyzed.unwrap());

    if let Err(error) = run_result {
        println!("Runtime Error: {}", error.message);
        return;
    }

    println!("{}", run_result.unwrap());
}
