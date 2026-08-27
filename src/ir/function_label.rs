
use crate::ast::Type;
use crate::ast::expr::Expr;
// use crate::ir::signature::Signature;
use crate::ir::type_mapping::type_mapping;
use crate::ast::block::Block;
use crate::ast::declarations::VaribleDecl;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLabel {
    pub function_name: String,
    // pub signature: Signature,
    pub variables: Vec<VaribleDecl>,  // 函数里面变量的声明。
    pub function_block: Option<Block>, // 函数代码块。

}
impl FunctionLabel {
    pub fn generate(&mut self) -> String {
        todo!()
    }

    pub fn get_native_stack_length(&self) -> usize {
        self.variables.len()
    }

    /// 局部生成变量。
    pub fn generate_a_native_variable(&self, variable: VaribleDecl) -> String {
        return variable.to_string()
    }
    

}
#[test]
fn test_generate_a_native_variable() {
    let function_label = FunctionLabel {
        function_name: "main".to_string(),
        variables: vec![],
        function_block: None,
    };
    let var = function_label.generate_a_native_variable(
        VaribleDecl {
             typ: Type::Int, 
             name: "NN".to_string(), 
             init: Some(Expr::Float(0.8)), 
            }
        );
    println!("{}", var);
}