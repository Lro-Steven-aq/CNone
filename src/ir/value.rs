use std::fmt::Display;

use crate::ir::types::Type;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    typ: Type,
    content: i128,
    extend: Option<u128>,
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        match value {
            Type::CHAR => Self { typ: value, content: 0, extend: None },
            Type::FLOAT => Self { typ: value, content: 0, extend: Some(0) },
            Type::INT => Self { typ: value, content: 0, extend: None },
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ {
            Type::CHAR => {
                debug_assert_eq!(self.extend, None);
                let ch = ( self.content as u8 ) as char;
                write!(f, "{}", ch)
            }
            Type::FLOAT => {
                debug_assert_ne!(self.extend, None);
                let mut result = String::default();
                let b = self.content.to_string();
                result.push_str(&b);
                result.push('.');
                let c = self.extend.unwrap().to_string();
                result.push_str(&c);
                write!(f, "{}", result)
            }
            Type::INT => {
                debug_assert_eq!(self.extend, None);
                let i = self.content;
                write!(f, "{}", i)
            }
        }
    }
}
impl Default for Value {
    fn default() -> Self {
        Self::new(Type::INT)
    }
}
impl Value {
    pub fn new(typ: Type) -> Self {
        if typ != Type::FLOAT {
            Self { typ, content: 0, extend: None }
        } else {
            Self { typ, content: 0, extend: Some(0) }
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        self.typ = typ;
    }
    pub fn set_value(&mut self, content: i128, extend: Option<u128>) {
        match self.typ {            // Guard
            Type::CHAR | Type::INT => {
                assert_eq!(extend, None);
            },
            Type::FLOAT => {
                assert_ne!(extend, None);
            },
        }
        self.content = content;
        self.extend = extend;
    }
    pub fn get_type(&self) -> Type {
        self.typ
    }
}

#[test]
#[should_panic]
fn test_value_display() {
    let mut m = Value::new(Type::FLOAT);
    println!("{}", m);
    m.set_type(Type::FLOAT);
    m.set_value(9, Some(8));
    println!("{}", m);
    m.set_value(1, None);   //   panic, assert error
}