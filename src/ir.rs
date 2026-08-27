mod code_generator;
mod function_label;
mod function_body;
mod type_mapping;
mod type_;
mod signature;

use std::collections::HashMap;

use crate::ast;
use crate::ast::declarations::Decl;
use crate::ast::program::Program;
use crate::ast::stmt::Stmt;

// use code_generator::CodeGenerator;
// use code_generator::clif_type;
// 
/// ## 用来编译程序到目标文件。
/// ### 缺失链接。
/// # Input cnone::ast::program::Program
/// # Return Vec<u8>
pub fn compile_program_to_bytecode(program: &Program) -> Vec<u8> {
    todo!()
}
