use lalrpop_util::lalrpop_mod;

pub mod analyzer;
pub mod ast;
pub mod builtins;
pub mod errors;
pub mod interpretor;
pub mod scopes;
pub mod types;
lalrpop_mod!(pub grammar);
