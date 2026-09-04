
use std::{cmp::max, fmt::Display, ops::{Add, Div, Mul, Sub}};

use num_integer::div_floor;

use crate::ir::types::Type;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    typ: Type,
    content: i128,
    extend: Option<u128>,
    length: Option<usize>,
}

// from i32/f32
// impl From <i128, i64, f64, ...>
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        let mut result = Value::new(Type::FLOAT);
        for (index, el) in value.to_string().split('.').enumerate() {
            if index == 0 {
                result.set_value(el.parse().unwrap(), Some(0), None);
            }
            if index == 1 {
                result.set_value(result.content, Some(el.parse().unwrap()), Some(el.len()));
            }
        }
        result
    }
}
impl From<Type> for Value {
    fn from(value: Type) -> Self {
        match value {
            Type::CHAR => Self { typ: value, content: 0, extend: None, length: None},
            Type::FLOAT => Self { typ: value, content: 0, extend: Some(0), length: Some(0) },
            Type::INT => Self { typ: value, content: 0, extend: None, length: None },
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
                if c.len() < self.length.unwrap() {
                    // /////
                    let _v = self.length.unwrap() - c.len();
                    for i in 0..self.length.unwrap() {
                        if i >= _v {
                            let _i = i - _v;
                            result.push_str(&self.extend.unwrap().to_string());
                            break;
                        } else {
                            result.push('0');
                        }
                    }
                    // /////
                } else if c.len() == self.length.unwrap() {
                    result.push_str(&c);
                } else {
                    panic!("Length is not enough to endure this float. /长度不够");
                }

                // result.push_str(&c);
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
impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::new(Type::CHAR);
        match self.typ {
            Type::CHAR | Type::INT => {
                if rhs.typ == Type::FLOAT {
                    result.set_type(Type::FLOAT);
                    result.set_value(self.content + rhs.content, rhs.extend, rhs.length);
                } else {
                    result.set_type(Type::INT);  // thought as int + char = int, int + int = int
                    result.set_value(self.content + rhs.content, None, None);
                }
            }
            Type::FLOAT => {
                if rhs.typ != Type::FLOAT {  
                    result.set_type(Type::FLOAT);     //  float + int = float   float + char = float
                    result.set_value(self.content + rhs.content, self.extend, self.length);
                } else {
                    result.set_type(Type::FLOAT);
                    if self.extend.unwrap() + rhs.extend.unwrap() < 10{
                        result.set_value(self.content + rhs.content, Some(self.extend.unwrap() + rhs.extend.unwrap()), Some(max(self.length.unwrap(), rhs.length.unwrap())));
                    } else {
                        result.set_value(self.content + rhs.content + 1, Some(self.extend.unwrap() + rhs.extend.unwrap() - 10), Some(max(self.length.unwrap(), rhs.length.unwrap())));
                    }
                }
            }
        }
        result
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::default();
        match self.typ {
            Type::CHAR | Type::INT => {
                if rhs.typ == Type::FLOAT {
                    result.set_type(Type::FLOAT);
                    result.set_value(self.content - rhs.content, rhs.extend, rhs.length);
                } else {
                    result.set_type(Type::INT);
                    result.set_value(self.content - rhs.content, None, None);
                }
            }
            Type::FLOAT => {
                if rhs.typ != Type::FLOAT {
                    result.set_type(Type::FLOAT);
                    result.set_value(self.content - rhs.content, self.extend, self.length);
                } else {
                    result.set_type(Type::FLOAT);
                    if self.extend.unwrap() >= rhs.extend.unwrap(){
                        result.set_value(self.content - rhs.content, Some(self.extend.unwrap() - rhs.extend.unwrap()), Some(max(self.length.unwrap(), rhs.length.unwrap())));
                    } else {
                        result.set_value(self.content - rhs.content + 1, Some(self.extend.unwrap() + 10 - rhs.extend.unwrap()), Some(max(self.length.unwrap(), rhs.length.unwrap())));
                    }
                }
            }
        }
        result
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        // regard character as integer.
        let mut is_self_char = false;
        let mut is_rhs_char = false;
        if self.typ == Type::CHAR{
            is_self_char = true;
            self.typ = Type::INT;
        }
        if rhs.typ == Type::CHAR {
            is_rhs_char = true;
            rhs.typ = Type::INT;
        }
        let self_num = self.to_string();
        let  rhs_num= rhs.to_string();
        let result = Self::from(self_num.parse::<f64>().unwrap() * rhs_num.parse::<f64>().unwrap());
        
        if is_self_char {self.set_type(Type::CHAR);}
        if is_rhs_char {rhs.set_type(Type::CHAR);}
        
        result
    }
}
impl Div for Value {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        // regard character as integer.
        let mut is_self_char = false;
        let mut is_rhs_char = false;
        if self.typ == Type::CHAR{
            is_self_char = true;
            self.typ = Type::INT;
        }
        if rhs.typ == Type::CHAR {
            is_rhs_char = true;
            rhs.typ = Type::INT;
        }
        let self_num = self.to_string();
        let  rhs_num= rhs.to_string();
        let result = Self::from(self_num.parse::<f64>().unwrap() / rhs_num.parse::<f64>().unwrap());
        
        if is_self_char {self.set_type(Type::CHAR);}
        if is_rhs_char {rhs.set_type(Type::CHAR);}
        
        result
    }
}
// impl PartialEq for Value {
//     fn eq(&self, other: &Self) -> bool {
//         self.content == other.content && 
//         self.extend == other.extend && 
//         self.length == other.length
//     }
// }
impl Eq for Value {}
impl PartialOrd for Value {
    fn ge(&self, other: &Self) -> bool {
        let mut _self = self.clone();
        let mut _other = other.clone();
        let mut is_self_char = false;
        let mut is_other_char = false;

        if _self.typ == Type::CHAR {
            is_self_char = true;
            _self.typ = Type::INT;
        }
        if _other.typ == Type::CHAR {
            is_other_char = true;
            _other.typ = Type::INT;
        }

        let result = _self.to_string().parse::<f64>().unwrap() >= _other.to_string().parse::<f64>().unwrap();
        if is_self_char {_self.typ = Type::CHAR;}
        if is_other_char {_other.typ = Type::CHAR;}
        result
    }
    fn lt(&self, other: &Self) -> bool {
      !self.ge(other)  
    }
    fn gt(&self, other: &Self) -> bool {
        let mut _self = self.clone();
        let mut _other = other.clone();
        let mut is_self_char = false;
        let mut is_other_char = false;

        if _self.typ == Type::CHAR {
            is_self_char = true;
            _self.typ = Type::INT;
        }
        if _other.typ == Type::CHAR {
            is_other_char = true;
            _other.typ = Type::INT;
        }

        let result = _self.to_string().parse::<f64>().unwrap() > _other.to_string().parse::<f64>().unwrap();
        if is_self_char {_self.typ = Type::CHAR;}
        if is_other_char {_other.typ = Type::CHAR;}
        result
    }
    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self > other {
            Some(std::cmp::Ordering::Greater)
        } else if self < other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

// impl ToString for Value {
//     fn to_string(&self) -> String {
//         let mut globalresult_ = String::new();
//         match self.typ {
//             Type::CHAR => {
//                 debug_assert_eq!(self.extend, None);
//                 let ch = ( self.content as u8 ) as char;
//                 globalresult_ = ch.to_string();
//             }
//             Type::FLOAT => {
//                 debug_assert_ne!(self.extend, None);
//                 let mut result = String::default();
//                 let b = self.content.to_string();
//                 result.push_str(&b);
//                 result.push('.');
//                 let c = self.extend.unwrap().to_string();
//                 if c.len() < self.length.unwrap() {
//                     // /////
//                     let _v = self.length.unwrap() - c.len();
//                     for i in 0..self.length.unwrap() {
//                         if i >= _v {
//                             let _i = i - _v;
//                             result.push_str(&self.extend.unwrap().to_string());
//                             break;
//                         } else {
//                             result.push('0');
//                         }
//                     }
//                     // /////
//                 } else if c.len() == self.length.unwrap() {
//                     result.push_str(&c);
//                 } else {
//                     panic!("Length is not enough to endure this float. /长度不够");
//                 }
//
//                 // result.push_str(&c);
//                 globalresult_ = result;
//             }
//             Type::INT => {
//                 debug_assert_eq!(self.extend, None);
//                 let i = self.content;
//                 globalresult_ = i.to_string();
//             }
//         }
//         globalresult_
//     }
// }

impl Value {
    pub fn new(typ: Type) -> Self {
        if typ != Type::FLOAT {
            Self { typ, content: 0, extend: None, length: None }
        } else {
            Self { typ, content: 0, extend: Some(0), length: Some(1) }
        }
    }
    pub fn set_type(&mut self, typ: Type) {
        if self.typ != Type::FLOAT && typ == Type::FLOAT {
            self.length = Some(0);
        }
        self.typ = typ;
    }
    pub fn set_value(&mut self, content: i128, extend: Option<u128>, length: Option<usize>) {
        let mut length = length;
        if length == None && self.typ == Type::FLOAT {
            length = self.length;
        }
        match self.typ {            // Guard
            Type::CHAR | Type::INT => {
                assert_eq!(extend, None);
                assert_eq!(length, None);
            },
            Type::FLOAT => {
                assert_ne!(extend, None);
                assert_ne!(length, None);
            },
        }
        self.content = content;
        self.extend = extend;
        self.length = length;
    }
    pub fn get_type(&self) -> Type {
        self.typ
    }
    pub fn get_extend(&self) -> Option<u128> {
        self.extend
    }
    pub fn get_content(&self) -> i128 {
        self.content
    }
    
}

#[test]
#[should_panic]
fn test_value_display() {
    let mut m = Value::new(Type::FLOAT);
    println!("{}", m);
    m.set_type(Type::FLOAT);
    m.set_value(9, Some(8), None);
    println!("{}", m);
    m.set_value(1, None, None);   //   panic, assert error
}
#[test]
fn test_value_operations() {
    let mut m = Value::new(Type::FLOAT);
    let mut n = Value::new(Type::FLOAT);
    m.set_value(10, Some(9), None); // m = 10.9
    n.set_value(8, Some(9), None); // n = 8.9

    // 加法
    debug_assert_eq!(m + n, 
        Value { typ: Type::FLOAT, content: 19, extend: Some(8), length: Some(1)});
    // 减法。----------------------------->>>>>>>>没有退位
    debug_assert_eq!(m - n, Value { typ: Type::FLOAT, content: 2, extend: Some(0), length: Some(1)});
    // from f64
    debug_assert_eq!(Value::from(2.00), 
        Value { content:2, extend: Some(0), typ: Type::FLOAT, length: Some(1)});
    // mul
    debug_assert_eq!(m * n, 
        Value {content: 97, typ: Type::FLOAT, extend: Some(1), length: Some(2)});
    m.set_value(8, Some(4), Some(1));
    n.set_value(2, Some(1), None);
    debug_assert_eq!(m / n, 
        Value {content: 4, typ: Type::FLOAT, extend: Some(0), length: Some(1)});
    debug_assert!(m > n);
    
}
