use super::operators::BinaryOp;
use super::operators::UnaryOp;
use crate::lexer::types::Type;

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Identifier(String),
    BinaryExp(BinaryOp, Box<Expr>, Box<Expr>),  //expr1 binop expr2
    UnaryExp(UnaryOp, Box<Expr>),     //unaryop expr
    Assign(Box<Expr>, Box<Expr>),
    CallFunction(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, String),
    PointerMember(Box<Expr>, String),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),   //三元运算符
    SizeOf(Type),
}