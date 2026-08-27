//! ### 编译器模块，提供IR到机器码的入口和JIT的入口。
//! 门面函数接口：compile(ir: Vec<u8>)
//! 由于其之前步骤的必要性缺失，我们即便实现了mod compiler，也难以调用C运行时标准库
//! 我们计划将其逐步实现。
//!

use std::fs;
use std::process::Command;

use crate::ast::program::Program;
use crate::ir::compile_program_to_bytecode;

pub fn compile(program: &Program, output_path: &str, remove_object_file: bool) {
    let bytecode = compile_program_to_bytecode(program);
    let object_file_output_path = format!("{}.o", output_path);
    fs::write(&object_file_output_path, bytecode).expect("Cannot open the path.");
    let status = Command::new("cc") // 连接器应该使用参数+默认值
        .arg(&object_file_output_path)
        .arg("-o")
        .arg(output_path)
        .arg("-no-pie")
        .status()
        .expect("Error to run linker");
    if !status.success() {
        panic!("Linking failed !");
    }
    if remove_object_file {
        if let Ok(status) = Command::new("rm")
            .arg("-r")
            .arg(object_file_output_path)
            .status()
        {
            if !status.success() {
                eprintln!("Error to remove the object file");
            }
        }
    }
}
