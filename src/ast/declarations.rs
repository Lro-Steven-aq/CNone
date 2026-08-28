use super::block::Block;
use super::expr::Expr;
// use super::field::Field;
use super::param::Param;
use crate::ast::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    // 总声明，包括了函数和变量。
    Function(FunctionDecl),
    Varible(VaribleDecl),
    // Struct(StructDecl),
    // TypeDef(TypeDefDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VaribleDecl {
    pub typ: Type,
    pub name: String,
    pub init: Option<Expr>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct StructDecl {
//     pub name: String,
//     pub fields: Vec<Field>,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct TypeDefDecl {
//     pub typ: Type,
//     pub alias: String,
// }
