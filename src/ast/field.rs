use crate::ast::Type;


#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub typ: Type,
}
