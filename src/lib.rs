use lalrpop_util::lalrpop_mod;

pub mod analyzer;
pub mod ast;
pub mod interpretor;
lalrpop_mod!(pub grammar);
