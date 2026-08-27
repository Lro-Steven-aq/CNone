use std::collections::HashMap;

use crate::ast::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct StructInfo {
    pub size: u32,
    pub align: u32,
    pub fields: HashMap<String, (Type, u32)>,
}
