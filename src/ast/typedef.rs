use crate::ast::Type;

#[derive(Debug, Clone,PartialEq)]
pub struct TypeDefDecl {
    pub alias: String,
    pub typ: Type,
}