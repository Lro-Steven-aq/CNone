use std::collections::HashMap;

use crate::ast::Type;

pub struct StructInfo {
    pub size: u32,
    pub align: u32,
    pub ields: HashMap<String, (Type, u32)>,// {"<字段名>": ["<字段类型>", "<偏移量>"]}
}