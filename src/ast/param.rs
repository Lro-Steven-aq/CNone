use crate::ast::Type;

#[derive(Debug,Clone,PartialEq)]
pub struct Param {
    pub typ: Type,
    pub name: String,
}