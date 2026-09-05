//!
//! 指令集
//! 此后mainloop里面只需要
//! ```ignore
//! for instruction in instructions{
//!     match instruction {
//!         Instruction::PUSH(typ, val) => {stack.push(new_value(typ,val));}
//!     }
//! }
//! ```
use crate::ir::{function_label::FunctionLabel, types::Type, value::Value};


pub type FunctionName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PUSH(Type, i128),   //  PUSH INT 10
    POP,
    LOAD(usize),
    STORE(Value, usize),
    GT,
    GE,
    LT,
    LE,
    CALL(FunctionName),
    JUMP(FunctionLabel),
    JUMPIF(FunctionLabel),
    ADD,
    SUB,
    MUL,
    DIV,
    INC,
    DEC,
    CLEAR,
    #[allow(non_camel_case_types)]
    _FUNCTION_LABEL__(FunctionName),
    // ......
}