use crate::ast::{
    block::Block, declarations::{
        Decl, 
        FunctionDecl, 
        VaribleDecl
    }, expr::Expr, program::Program, stmt::Stmt
};


#[derive(Debug, Clone, PartialEq)]
struct CodeGenerator {
    depth: usize,
    is_function: bool,
    is_variable: bool,
    program: Program,
    index: usize,

}
impl CodeGenerator {
    pub fn new(program: Program) -> Self {
        Self { depth: 0, is_function: false, is_variable: false, program: program, index: 0 }
    }

    pub fn generate(&mut self) -> String {
        let mut result = String::default();
        for i in self.program.decls.clone() {
            if let Decl::Function(function) = i {
                result.push_str(&self.generate_function(function));
                if self.is_variable {
                    self.depth += 1;
                    self.is_function = true;
                    self.is_variable = false;
                }
            } else if let Decl::Varible(variable) = i {
                result.push_str(&self.generate_variable(variable));
                if self.is_function {
                    self.depth += 1;
                    self.is_function = false;
                    self.is_variable = true;
                }
            } else {
                unreachable!();    
            }
            self.index += 1;
        }
        result
    }

    fn generate_function(&mut self, function: FunctionDecl) -> String {
        let name = function.name;
        let body = function.body.unwrap_or_else(||{
            Block {stmts: Vec::new()}
        });
        format!(
            "{}:\n{}",
            name,
            self.generate_block(body), // Function Body
        )
    }

    fn generate_block(&mut self, block: Block) -> String {
        let mut result = String::default();
        for stmt in block.stmts {
            match stmt {
                Stmt::VaribleDecl(variable) => {
                    result.push_str(&self.generate_variable(variable));
                    if self.is_function {
                        self.depth +=1;
                        self.is_function = false;
                        self.is_variable = true;
                    }
                }
                Stmt::Block(_block) => {
                    result.push_str(&self.generate_block(_block));
                    // // // // // // // // // // // // // // // // // // // //
                }
                Stmt::While(condition, body) => {
                    result.push_str(&self.generate_while(condition, body));
                }
                Stmt::Return(value) => {
                    result.push_str(&self.generate_return(value));
                    unimplemented!()
                }
                Stmt::If(condition, _then, _else) => {
                    result.push_str(&self.generate_if(condition, _then, _else));
                }
                Stmt::For(init, condition, step, stmt) => {
                    result.push_str(&self.generate_for(init, condition, step, stmt));
                }
                Stmt::Expr(expr) => {
                    result.push_str(&self.generate_expr(expr));
                }
                _ => unimplemented!()
            }
        }
        result
    }
    
    fn generate_variable(&self, variable: VaribleDecl) -> String {
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




///
/// for (int i = 0; i < 10; i++) {
///     echo(i)
/// }
/// ///////////////////////////////
/// Implements:
/// ///////////////////////////////
/// MAIN:
///     PUSH INT 0          [0]
///     COPY                [0,0]
///     PUSH INT 10         [0,0,10]
/// loop_body:
///     LT                  [0,1]   [0,1]
///     JUMPIF true_label   [0,1]
/// 
/// true_label:
///     PUSH INT 10         [0,1,10]
///     LT                  [0,1]
///     JUMPIF loop_body
/// 
/// ## []
impl CodeGenerator {
    fn generate_while(&mut self, condition: Expr, body: Box<Stmt>) -> String {
        // format!("JUMP\n{}\n")
        unimplemented!()
    }
    fn generate_return(&mut self, value: Option<Expr>) -> String {
        unimplemented!()
    }
    fn generate_if(&mut self, condition: Expr, then_block: Box<Stmt>, else_block: Option<Box<Stmt>>) -> String {
        unimplemented!()
    }
    fn generate_for(&mut self, init: Option<Expr>, condition: Option<Expr>, step: Option<Expr>, body: Box<Stmt>) -> String {
        unimplemented!()
    }
    fn generate_expr(&mut self, expr: Expr) -> String {
        unimplemented!()
    }
}