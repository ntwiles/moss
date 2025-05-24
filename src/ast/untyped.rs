use crate::typing::ProtoType;

use super::Span;

#[derive(Clone, Debug)]
pub enum Expr {
    // Binary operations
    Eq(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gte(Box<Expr>, Box<Expr>),
    Lte(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),

    // Unary operations
    Negate(Box<Expr>),
    Assignment {
        ident: String,
        expr: Box<Expr>,
    },
    Declaration {
        ident: String,
        type_annotation: Option<ProtoType>,
        expr: Box<Expr>,
        is_mutable: bool,
    },

    // Postfix operations
    FuncCall(FuncCall, Span),

    // Control flow
    If(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Stmt>, Span),
    Loop(Box<Expr>),
    Break,

    // Primaries
    Literal(Literal),
    Identifier(String),
    FuncDeclare(FuncDeclare),
    List(Vec<Expr>),
}

impl Expr {
    pub fn is_func_declare(&self) -> bool {
        return matches!(self, Expr::FuncDeclare(_));
    }

    pub fn as_func_declare(&self) -> &FuncDeclare {
        if let Expr::FuncDeclare(func) = self {
            &func
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct FuncCall {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct FuncDeclare {
    pub params: Vec<(String, ProtoType)>,
    pub return_type: Box<ProtoType>,
    pub block: Box<Expr>,
    pub is_closure: bool,
}
