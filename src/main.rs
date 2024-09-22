mod analyzer;
mod ast;
mod interpretor;
lalrpop_mod!(grammar);

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
            println!("Error loading nala file: {}", err);
            return;
        }
    };

    let parsed = ProgramParser::new().parse(&code).unwrap();

    let analyzed = analyzer::analyze_expr(&parsed);
    let result = interpretor::Interpreter::new().eval(analyzed);

    println!("{}", result);
}
