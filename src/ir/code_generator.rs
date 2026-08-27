use std::collections::HashMap;

use crate::ast::declarations::Decl;
use crate::ast::program::Program;

/// # 代码生成器。
/// ### 不直接暴露出mod ast 外部应该使用compile_program来间接调用。
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenerator {
    source: Program,

}

impl CodeGenerator {
    pub fn new(source: Program ) -> Self {
        Self { source: source }
    }
    pub fn generate(&self) -> String {
        // let functions = Vec<FunctionLabel>::new();
        // for decl in self.source.decls {
        //     match decl {
        //         Decl::Function(function) => {
        //             let function_label = self.generate_function(function);
        //         }
        //         Decl::Varible(variable) => {
        //             let label = self.generate_variable(variable);
        //         }
        //     }

        // }
        todo!()
    }

}