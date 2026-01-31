#[derive(Debug)]
pub enum Expr {
    Number(u64),
    Ident(String),
    BinOp(Box<Expr>, BinOpType, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug)]
pub enum BinOpType {
    Plus,
    Minus,
    Le,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    Return(Expr),
    Out(Expr),
    FuncDef(String, String, Vec<Stmt>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
