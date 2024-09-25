use lalrpop_util::lalrpop_mod;

pub mod analyzer;
pub mod ast;
pub mod errors;
pub mod interpretor;
mod scope_stack;
lalrpop_mod!(pub grammar);
