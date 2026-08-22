
use std::collections::HashMap;

use cranelift::codegen::ir::FuncRef;
use cranelift::prelude::*;
use cranelift_module::{Module, Linkage, FuncId};
use cranelift_object::ObjectModule;
use cranelift_object::ObjectBuilder;
use target_lexicon::triple;

use crate::ast;
use crate::ast::expr::Expr;
use crate::ast::operators::{BinaryOp, UnaryOp};
use crate::ast::program::Program;
use crate::ast::stmt::Stmt;
pub struct CodeGenerator<'a, MODULE: Module> {
    module: &'a mut MODULE,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    function_identifiers: HashMap<String, FuncId>,
}

impl<'a, MODULE: Module> CodeGenerator<'a, MODULE> {
    //  映射 cnone::ast::Type -> Cranelift::Type
    fn clif_type(&self, typ: &ast::Type) -> Type {
        match typ {
            ast::Type::Int | ast::Type::Long => types::I32,
            ast::Type::Float => types::F32,
            ast::Type::Double => types::F128,   // 本来应该是F64，但是我想给它大一点
            ast::Type::Char => types::I8,
            ast::Type::Void => types::INVALID,
            _ => types::I32,
        }
    }

    /// 分配变量内存地址。
    fn declare_variable(&mut self, name: &str, typ: Type) -> Variable {
        let variable = Variable::new(self.variables.len());
        self.builder.declare_var(variable, typ);
        self.variables.insert(name.to_string(), variable);
        variable
    }

    /// ---------------AST遍历生成。--------------------------
    /// 1.语句。
    pub fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VaribleDecl(decl) => {
                let typ = self.clif_type(&decl.typ);
                let var = self.declare_variable(&decl.name, typ);
                if let Some(init_value) = &decl.init {
                    let value = self.generate_expr(init_value);
                    self.builder.def_var(var, value);
                }
            },
            Stmt::Expr(expr) => {
                self.generate_expr(expr);
            },
            Stmt::Return(Some(expr)) => {
                let value = self.generate_expr(expr);
                self.builder.ins().return_(&[value]);
            },
            Stmt::Return(None) => {
                self.builder.ins().return_(&[]);
            },
            Stmt::Block(block) => {
                for stmt in &block.stmts {
                    self.generate_stmt(stmt);
                }
            },
            Stmt::If(condition,then_branch ,else_branch ) => {
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let merge_block = self.builder.create_block();
                let condition_value = self.generate_expr(condition);
                self.builder.ins().brif(
                    condition_value,
                     then_block, &[],
                      else_block, &[]
                    );
                // then 块的代码。
                self.builder.switch_to_block(then_block);
                self.generate_stmt(then_branch);
                if !self.is_block_filled() {
                    self.builder.ins().jump(merge_block, &[]);
                }

                //  else 块的代码。
                self.builder.switch_to_block(else_block);
                if let Some(else_stmt) = else_branch {
                    self.generate_stmt(else_stmt);
                }
                if !self.is_block_filled() {
                    self.builder.ins().jump(merge_block, &[]);
                }

                // merge 块的代码。
                self.builder.switch_to_block(merge_block);
                self.builder.seal_block(then_block);
                self.builder.seal_block(else_block);
                self.builder.seal_block(merge_block);

            },
            _ => todo!("更多的即将实现。。。(while, for, continue)"),

        }
    }

    fn is_block_filled(&self) -> bool {
        self.builder.is_filled()   // private method error!
    }
    
    // 2.遍历表达式。
    pub fn generate_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(i) => {
                self.builder.ins().iconst(types::I32, *i)
            },
            Expr::Float(i) => {
                self.builder.ins().f32const(*i as f32)
            },
            Expr::Identifier(name) => {
                let var = self.variables.get(name).expect(
                    &format!("Undefined variable: {}", name).to_string()
                );
                self.builder.use_var(*var)
            },
            Expr::BinaryExp(op, left ,right ) => {
                let left = self.generate_expr(left);
                let right = self.generate_expr(right);
                self.generate_binary_operator(op, left, right)
            },
            Expr::UnaryExp(op ,obj ) => {
                let value = self.generate_expr(obj);
                match op {
                    UnaryOp::Neg => self.builder.ins().ineg(value),
                    _ => todo!("更多的即将实现。。。(UnaryOp)"),
                }
            },
            Expr::Assign(left, right) => {
                let value = self.generate_expr(right);
                match left.as_ref() {
                    Expr::Identifier(name) => {
                        let var = self.variables.get(name).unwrap();
                        self.builder.def_var(*var, value);
                        value
                    },
                    _ => panic!("Invalid assignment target."),
                }
            },
            Expr::CallFunction(function_anme, args ) => {
                let mut args: Vec<Value> = args.iter()
                    .map(|arg|self.generate_expr(arg))
                    .collect();
                let function_ref = self.get_function_reference(function_anme);
                let call = self.builder.ins().call(function_ref, &args);
                self.builder.inst_results(call)[0]
            },
            _ => todo!("更多的即将实现。。。"),
        }
    }

    fn generate_binary_operator(&mut self, op: &BinaryOp, left: Value, right: Value) -> Value {
        let inst = self.builder.ins();
        match op {
            BinaryOp::Add => inst.iadd(left, right),
            BinaryOp::Sub => inst.isub(left, right),
            BinaryOp::Mul => inst.imul(left, right),
            BinaryOp::Div => inst.sdiv(left, right),
            BinaryOp::Mod => inst.srem(left, right),
            BinaryOp::Eq => inst.icmp(IntCC::Equal, left, right),
            BinaryOp::Ne => inst.icmp(IntCC::NotEqual, left, right),
            BinaryOp::Lt => inst.icmp(IntCC::SignedLessThan, left, right),
            BinaryOp::Le => inst.icmp(IntCC::SignedLessThanOrEqual, left, right),
            BinaryOp::Gt => inst.icmp(IntCC::SignedGreaterThan, left, right),
            BinaryOp::Ge => inst.icmp(IntCC::SignedGreaterThanOrEqual, left, right),
            BinaryOp::And => inst.band(left, right),
            BinaryOp::Or => inst.bor(left, right),
            _ => todo!("更多的即将实现。。。（剩下5个位运算）"),
        }
    }

    fn get_function_reference(&mut self, name: &str) -> FuncRef {
        let function_id = self.function_identifiers.get(name).copied().expect(format!("Undefined Function: {}", name));
        self.module.declare_func_in_func(function_id, &mut self.builder.func)
    }

}


pub fn compile_program(program: &Program) -> Vec<u8> {
    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(triple!("x86_64-unknown-linux-gnu")).unwrap();
    let builder = ObjectBuilder::new(isa, "main", cranelift_module::default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);
    ///////////////////////////////////////////////////////////
    /// TODO: 声明所有的函数，然后给予其定义，再，，，             ///
    ///////////////////////////////////////////////////////////
}