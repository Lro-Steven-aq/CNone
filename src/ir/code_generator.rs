
mod error;
use std::collections::HashMap;

use crate::{ast::{
    block::Block, declarations::{
        Decl, 
        FunctionDecl, 
        VaribleDecl
    }, expr::Expr, program::Program, stmt::Stmt
}, ir::{function_label::FunctionLabel, instructions::FunctionName, variable_slot_table::Slot}};
use crate::ir::instructions::Instruction;
use crate::vm::variables::VariablesSlotTable;
use error::Error;

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
struct CodeGenerator {
    program: Program,
    // index: usize,

    bytecode: Vec<Instruction>,       // Output.
    slot_table: VariablesSlotTable,   // slot -> value
    symbols: HashMap<String, Slot>,   // variable_name -> slot
    functions: HashMap<FunctionName, FunctionLabel>,  // function_name -> function_label

}
impl CodeGenerator {
    pub fn new(program: Program) -> Self {
        Self { 
            program: program, 
            // index: 0, 
            bytecode: Vec::new(),
            slot_table: VariablesSlotTable::new(),
            symbols: HashMap::new(),
            functions: HashMap::new(),
         }
    }

    pub fn generate(&mut self) -> Result<Vec<Instruction>> {
        for decl in self.program.decls.clone() {
            match decl {
                Decl::Function(function) => self.generate_function(function)?,
                Decl::Varible(variable) => self.generate_variable(variable)?,
            }
        }
        self.finish()
    }

    fn finish(&mut self) -> Result<Vec<Instruction>> {
        Ok(self.bytecode.clone())
    }
    fn generate_function(&mut self, function: FunctionDecl) -> Result<()> {
        let entry = self.bytecode.len();
        let mut label = FunctionLabel::new(entry, 
            self.bytecode.clone(), 
            function.name.clone());
        self.functions.insert(function.name.clone(), label);
        for (index, param) in function.params.iter().enumerate() {
            // self.symbols.insert(param.clone(), v)
        }
        unimplemented!()
    }
    
    fn generate_variable(&self, variable: VaribleDecl) -> Result<()> {
        unimplemented!()
    }
}
/*
例如：
MAIN:
    # START
    PUSH INT 1
    PUSH INT 2              # 前面有类型的，是数字或字符；否则是操作数栈的索引。
    ADD
    CALL ECHO
    PUSH INT 4
    GT
    JUMP X
X:
    JUMP W
    CALL ECHO
W:
    CALL ECHO
*/




// 
//  for (int i = 0; i < 10; i++) {
//      echo(i)
//  }
//  ///////////////////////////////
//  Implements:
//  ///////////////////////////////
//  MAIN:
//      PUSH INT 0          [0]
//      COPY                [0,0]
//      PUSH INT 10         [0,0,10]
//  loop_body:
//      LT                  [0,1]   [0,1]
//      JUMPIF true_label   [0,1]
//  
//  true_label:
//      PUSH INT 10         [0,1,10]
//      LT                  [0,1]
//      JUMPIF loop_body
//  
//  ## []
