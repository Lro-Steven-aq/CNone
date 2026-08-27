use crate::{ast::Type, ir::type_::IRType};


pub fn type_mapping(typ: &Type) -> IRType {
    match typ {
        Type::Char => IRType::CHAR,
        Type::Double | Type::Float => IRType::FLOAT,
        Type::Int | Type::Long |
        Type::Short | Type::Signed | 
        Type::Pointer(_) => IRType::INT,
        Type::Unsigned | Type::Void=> IRType::UINT,
        Type::Struct(_) => panic!("Unsupport Struct"),
    }
}