use std::fmt::{Display, write};

//记录类型。指的是被分析代码的变量的.
/*
        比如说：
            ```c
            int a = 0;
            int c = 9;
            float b = 2.9;
            ```
        中的 "int","float"
        而非是Token（
            Keyword(int),
            Identifer("a"),
            Operater(Assign),
            ...）的类型。
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Float,
    Double,
    Short,
    Long,
    Unsigned,
    Signed,
    Void,

    Struct(String),

    Pointer(Box<Type>), //  不是词法分析应该解析的类型，放在语法分析时处理。
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Char => write!(f, "CHAR"),
            Type::Float | Type::Double => write!(f, "FLOAT"),
            Type::Int | Type::Short |
            Type::Long | Type::Signed => write!(f, "INT"),
            Type::Unsigned | Type::Void => write!(f, "UINT"),
            Type::Pointer(_) | Type::Struct(_) => panic!("Unsupport"),
        }
    }
}