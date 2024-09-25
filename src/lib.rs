use lalrpop_util::lalrpop_mod;

pub mod analyzer;
pub mod ast;
pub mod errors;
pub mod interpretor;
pub mod shared;
lalrpop_mod!(pub grammar);
