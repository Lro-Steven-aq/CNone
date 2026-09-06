
mod error;
use std::collections::HashMap;

use crate::{ast::{
    Type, block::Block, declarations::{
        Decl, 
        FunctionDecl, 
        VaribleDecl
    }, expr::Expr, operators::BinaryOp, program::Program, stmt::Stmt
}, ir::{self,function_label::FunctionLabel, instructions::FunctionName, value::Value, variable_slot_table::Slot}};
use crate::ir::instructions::Instruction;
use crate::vm::variables::VariablesSlotTable;
use error::Error;

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
struct CodeGenerator {
    program: Program,
    index: usize,

    bytecode: Vec<Instruction>,       // Output.
    slot_table: VariablesSlotTable,   // slot -> value
    symbols: HashMap<String, Slot>,   // variable_name -> slot
    functions: HashMap<FunctionName, FunctionLabel>,  // function_name -> function_label
    // variable_types: HashMap<String, Type>,
}
impl CodeGenerator {
    pub fn new(program: Program) -> Self {
        Self { 
            program: program, 
            index: 0, 
            bytecode: Vec::new(),
            slot_table: VariablesSlotTable::new(),
            symbols: HashMap::new(),
            functions: HashMap::new(),
            // variable_types: HashMap::new(),
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
        let mut val = Value::new(ir::types::Type::CHAR);
        val.set_value(self.functions.len() as i128, None, None);
        let label = FunctionLabel::new(self.functions.len(), function.name.clone());
        self.functions.insert(function.name.clone(), label);

        if function.body != None {
            let block = function.body.unwrap();
            self.generate_block(block)?;
        }
        Ok(())
    }
    
    fn generate_variable(&mut self, variable: VaribleDecl) -> Result<()> {
        if self.symbols.contains_key(&variable.name) {
            return Err(Error { msg: "Existed Variable".to_string(), code: 7 });
        }
        self.index += 1;
        // define
        if let Some(init) = variable.init {
            self.generate_expr(init)?;
        } else {
            let default_value = match variable.typ {
                Type::Char => Value::new(ir::types::Type::CHAR),
                Type::Double | Type::Float => Value::new(ir::types::Type::FLOAT),
                Type::Int | Type::Long |
                Type::Short | Type::Char |
                Type::Noreturn => Value::new(ir::types::Type::INT),
            };
            self.bytecode.push(Instruction::PUSH(default_value));
            // ///////////////
            let slot = Slot { value: default_value, index: self.index };
            self.symbols.insert(variable.name.clone(), slot);
        }
        self.bytecode.push(Instruction::STORE(self.index));
        // end define
        
        // let slot = Slot { value: value, index: self.index };
        // self.symbols.insert(variable.name.clone(), slot);
        Ok(())
    }

    fn generate_block(&mut self, block: Block) -> Result<()> {
        for stmt in block.stmts {
            self.generate_stmt(stmt)?;
        }
        Ok(())
    }
    ///
    /// CORE
    fn generate_stmt(&mut self, stmt: Stmt) -> Result<()> {
        self.index += 1;
        match stmt {
            Stmt::Block(block) => self.generate_block(block)?,
            Stmt::Expr(expr) => self.generate_expr(expr)?,
            Stmt::If(condition, then_block, else_block) => self.generate_if(condition, then_block, else_block)?,
            Stmt::Return(expr) => self.generate_return(expr)?,
            Stmt::VaribleDecl(variable) => self.generate_variable(variable)?,
            Stmt::While(condition, body) => self.generate_while(condition, body)?,
            _ => return Err(Error { msg: "Unsupported Statement Exception".to_string(), code: 1}),
        }
        Ok(())
    }
    fn generate_expr(&mut self, expr: Expr) -> Result<()> {
        self.index += 1;
        match expr {
           Expr::Assign(variable, value) => {
                match *variable {
                    Expr::Identifier(name) => {
                        let mut slot = self.symbols.get(&name).ok_or_else(|| Error{msg: "Undefined Variable".to_string(), code: 2})?.clone();
                        self.generate_expr(*value)?;
                        self.bytecode.push(Instruction::PUSH(slot.value));
                        self.bytecode.push(Instruction::STORE(self.index));
                        self.slot_table.set_variable(self.index, slot.value);
                        slot.index = self.index;
                        self.index += 1;
                    }
                    _ => return Err(Error {msg: "Left expr should be a identifier".to_string(), code: 3}),
                }
           }
           Expr::BinaryExp(op, left, right) => {
                self.generate_expr(*left)?;
                self.generate_expr(*right)?;
                let opcode = match op {
                    BinaryOp::Add => Instruction::ADD,
                    BinaryOp::And => Instruction::MUL,
                    BinaryOp::Sub => Instruction::SUB,
                    BinaryOp::Mul => Instruction::MUL,
                    BinaryOp::Div => Instruction::DIV,
                    BinaryOp::Eq => Instruction::EQ,
                    BinaryOp::Ne => Instruction::NE,
                    BinaryOp::Gt => Instruction::GT,
                    BinaryOp::Ge => Instruction::GE,
                    BinaryOp::Lt => Instruction::LT,
                    BinaryOp::Le => Instruction::LE,
                    BinaryOp::Mod => Instruction::MOD,
                    _ => unimplemented!("Haven't implemented yet."),
                };
                self.bytecode.push(opcode);
           }
           Expr::Integer(integer) => {
                let mut value = Value::new(ir::types::Type::INT);
                value.set_value(integer as i128, None, None);
                self.bytecode.push(Instruction::PUSH(value));
           }
           Expr::Identifier(name) => {
                let slot = self.symbols.get(&name).ok_or(Error { msg: "Undefined Variable".to_string(), code: 4})?;
                self.bytecode.push(Instruction::LOAD(slot.index));
                self.index += 1;
           }
           Expr::CallFunction(function_name,args ) => {
                for arg in args {
                    self.generate_expr(arg)?;
                }
                if function_name == "ECHO".to_string() {
                    self.bytecode.push(Instruction::CALL(function_name));
                } else {
                    let function_label = FunctionLabel::new(self.index, function_name.clone());
                    self.bytecode.push(Instruction::JUMP(function_label));
                }
           }
           Expr::Char(ch) => {
                let mut value = Value::new(ir::types::Type::CHAR);
                value.set_value(ch as i128, None, None);
                self.bytecode.push(Instruction::PUSH(value));
           }
           Expr::Float(float) => {
                let value = Value::from(float);
                self.bytecode.push(Instruction::PUSH(value));
           }
           Expr::String(string) => {
                for ch in string.chars().rev() {    // 应该需要反转
                    self.generate_expr(Expr::Char(ch))?;
                }
           }
           _ => return Err(Error { msg: "Unsupported Expr".to_string(), code: 6}),

        }
        Ok(())
    }

    fn generate_if(&mut self, condition: Expr, then_block: Box<Stmt>, else_block: Option<Box<Stmt>>) -> Result<()> {
        unimplemented!()
    }
    
    fn generate_return(&mut self, expr: Option<Expr>) -> Result<()> {
        unimplemented!()
    }

    fn generate_while(&mut self, condition: Expr, body: Box<Stmt>) -> Result<()> {
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

#[test]
fn test_code_generator() {
    use std::fs;
    use crate::preprocessor::Preprocessor;
    use crate::lexer::Lexer;
    use crate::lexer::get_tokens;
    use crate::ast::Parser;
    let mut preprocesser = Preprocessor::new(fs::read_to_string("test/test.cnone").unwrap().as_str());
    let source = preprocesser.preprocess();
    let mut lexer = Lexer::new(source);
    let tokens = get_tokens(&mut lexer).unwrap();
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse();
    let mut generator = CodeGenerator::new(ast);
    println!("{:#?}",generator.generate().unwrap());
}