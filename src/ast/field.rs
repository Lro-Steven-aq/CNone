use crate::ast::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub typ: Type,
    pub name: String,
}