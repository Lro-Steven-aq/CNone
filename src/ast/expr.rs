use core::task;
use std::fmt::Display;

use super::operators::BinaryOp;
use super::operators::UnaryOp;
use crate::ast::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Identifier(String),
    BinaryExp(BinaryOp, Box<Expr>, Box<Expr>), //expr1 binop expr2
    UnaryExp(UnaryOp, Box<Expr>),              //unaryop expr
    Assign(Box<Expr>, Box<Expr>),
    CallFunction(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, String),
    PointerMember(Box<Expr>, String),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>), //三元运算符
    SizeOf(Type),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::to_string(self))
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Char(ch) => ch.to_string(),
            Expr::Float(f) => f.to_string(),
            Expr::Integer(i) => i.to_string(),
            _ => "#".to_string(),
        }
    }
    pub fn get_type_of(&self) -> Type {
        match self {
            Expr::Char(_) => Type::Char,
            Expr::Float(_) => Type::Float,
            Expr::Integer(_) => Type::Int,
            _ => Type::Void,
        }
    }
}