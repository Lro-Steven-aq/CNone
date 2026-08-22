use crate::lexer::types::Type;
use super::param::Param;
use super::block::Block;
use super::expr::Expr;

#[derive(Debug,Clone,PartialEq)]
pub enum Decl {             // 总声明，包括了函数和变量。
    Function(FunctionDecl),
    Varible(VaribleDecl),
}

#[derive(Debug,Clone,PartialEq)]
pub struct FunctionDecl {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Block>,
}

#[derive(Debug,Clone,PartialEq)]
pub struct VaribleDecl {
    pub typ: Type,
    pub name: String,
    pub init: Option<Expr>,
}
