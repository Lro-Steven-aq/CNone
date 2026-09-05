//! 这是函数标签，它代表一个函数/标签（非内置）
//! ## 这是一个 `For` 循环:
//! ```ignore
//! init:
//!     PUSH INT 0
//!     STORE 0
//!     JUMP __func__
//! func1:
//!     LOAD 0
//!     INC
//!     PUSH INT 10
//!     GE
//!     JUMPIF endif
//!     JUMP __func__
//! endif:
//! __func__：
//!     FUNCTION-BODY
//!     JUMP func1
//! ```

use crate::ir::{instructions::Instruction, variable_slot_table::Index};
use crate::ir::instructions::FunctionName;
// use super::Stmt;
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLabel {
    name: String,
    position: Index,  //   函数标签开始处。func1:
    instructions: Vec<Instruction>,
}

impl FunctionLabel {
    pub fn new(index: Index, instructions: Vec<Instruction>, name: String) -> Self {
        let name = instructions[index].clone();
        if let Instruction::_FUNCTION_LABEL__(function_name) = name {
            Self { name: function_name, position: index, instructions: instructions }
        } else {
            panic!("Error @Function Index")
        }
    }
    
    pub fn get_index(&self) -> usize {
        self.position
    }

    pub fn build(value: FunctionName) -> Self {
        panic!("You Should Call FunctionLabel::new externally");
    }
}

// impl From<FunctionName> for FunctionLabel {
//     fn from(value: FunctionName) -> Self {
//         // build it

//     }
// }