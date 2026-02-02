use crate::types::{IntegerType, IntegerValue, StringValue, Value};

#[derive(Debug)]
pub enum Expr {
    Number(IntegerValue),
    TypedNumber(IntegerValue),
    Ident(String),
    BinOp(Box<Expr>, BinOpType, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Hash)]
pub enum BinOpType {
    Plus,
    Minus,
    Mul,
    Mod,
    Le,
    Lt,
    Gt,
    Ge,
    Eq,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    Return(Expr),
    Out(Expr),
    FuncDef(String, Vec<String>, Vec<Stmt>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
