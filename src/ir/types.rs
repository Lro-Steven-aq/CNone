use crate::{ast};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    CHAR,
    INT,
    FLOAT,
}

impl From<ast::Type> for Type {
    fn from(value: ast::Type) -> Self {
        match value {
            ast::Type::Char => Type::CHAR,
            ast::Type::Float | ast::Type::Double => Type::FLOAT,
            ast::Type::Int | ast::Type::Long |
            ast::Type::Short | ast::Type::Signed |
            ast::Type::Unsigned | ast::Type::Void => Type::INT,
            ast::Type::Pointer(_) | ast::Type::Struct(_) => panic!("Unsupported type"),

        }
    }
}

#[test]
fn test_ir_type_from() {
    let typ = ast::Type::Double;
    // let __typ: Type = typ.into();
    assert_eq!(Type::from(typ), Type::FLOAT);
}