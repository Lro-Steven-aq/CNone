
use std::collections::HashMap;

use cranelift::codegen::ir::{FuncRef, StackSlot};
use cranelift::prelude::*;
use cranelift_module::{FuncId, Module};

use crate::ast;
use crate::ast::expr::Expr;
use crate::ast::operators::{BinaryOp, UnaryOp};
use crate::ast::stmt::Stmt;

/// # 代码生成器。
/// ### 不直接暴露出mod ast 外部应该使用compile_program来间接调用。
/// 
/// 
pub struct CodeGenerator<'a, MODULE: Module> {
    pub module: &'a mut MODULE,
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, StackSlot>,
    pub function_identifiers: HashMap<String, FuncId>,
    pub current_block_terminated: bool,
}

impl<'a, MODULE: Module> CodeGenerator<'a, MODULE> {
    //  映射 cnone::ast::Type -> Cranelift::Type
    fn clif_type(typ: &ast::Type) -> Type {
        clif_type(typ)
    }

    /// 分配变量内存地址。
    pub fn declare_variable(&mut self, name: &str, typ: Type) -> StackSlot {
        let slot = self.builder.create_sized_stack_slot(
            StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                typ.bytes(), 
                0
            )
        );
        self.variables.insert(name.to_string(), slot);
        slot
    }

    /// ---------------AST遍历生成。--------------------------
    /// 1.语句。
    pub fn generate_stmt(&mut self, stmt: &Stmt) {
        if self.current_block_terminated {
            return;
        }
        match stmt {
            Stmt::VaribleDecl(decl) => {
                let typ = Self::clif_type(&decl.typ);
                let slot = self.declare_variable(&decl.name, typ);
                if let Some(init_value) = &decl.init {
                    let value = self.generate_expr(init_value);
                    self.builder.ins().stack_store(value, slot, 0);
                }
            }
            Stmt::Expr(expr) => {
                self.generate_expr(expr);
            }
            Stmt::Return(Some(expr)) => {
                let value = self.generate_expr(expr);
                self.builder.ins().return_(&[value]);
                self.current_block_terminated = true;
            }
            Stmt::Return(None) => {
                self.builder.ins().return_(&[]);
                self.current_block_terminated = true;
            }
            Stmt::Block(block) => {
                for stmt in &block.stmts {
                    self.generate_stmt(stmt);
                    if self.current_block_terminated {
                        break;
                    }
                }
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let merge_block = self.builder.create_block();
                let condition_value = self.generate_expr(condition);
                self.builder
                    .ins()
                    .brif(condition_value, then_block, &[], else_block, &[]);
                // then 块的代码。
                self.builder.switch_to_block(then_block);
                self.current_block_terminated = true;
                self.generate_stmt(then_branch);
                if !self.current_block_terminated {
                    self.builder.ins().jump(merge_block, &[]);
                }
                self.builder.seal_block(then_block);

                //  else 块的代码。
                self.builder.switch_to_block(else_block);
                self.current_block_terminated = false;
                if let Some(else_stmt) = else_branch {
                    self.generate_stmt(else_stmt);
                }
                if !self.current_block_terminated {
                    self.builder.ins().jump(merge_block, &[]);
                }
                self.builder.seal_block(else_block);

                // merge 块的代码。
                self.builder.switch_to_block(merge_block);
                self.current_block_terminated = true;
                self.builder.seal_block(merge_block);
            }
            _ => todo!("更多的即将实现。。。(while, for, continue)"),
        }
    }

    pub fn is_block_filled(&self) -> bool {
        self.current_block_terminated // private method error!
    }

    // 2.遍历表达式。
    pub fn generate_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(i) => self.builder.ins().iconst(types::I32, *i),
            Expr::Float(i) => self.builder.ins().f32const(*i as f32),
            Expr::Identifier(name) => {
                let slot = self.variables.get(name).unwrap();
                self.builder.ins().stack_load(types::I32, *slot, 0) // 类型后面改。
            }
            Expr::BinaryExp(op, left, right) => {
                let mut left = self.generate_expr(left);
                let mut right = self.generate_expr(right);

                let left_type = self.builder.func.dfg.value_type(left);
                let right_type = self.builder.func.dfg.value_type(right);
                //  统一为大类型。
                if left_type != right_type {
                    let target_type = if left_type.bytes() > right_type.bytes() {
                        left_type
                    } else {
                        right_type
                    };
                    if left_type != target_type {
                        left = self.builder.ins().uextend(target_type, left);
                    }
                    if right_type != target_type {
                        right = self.builder.ins().uextend(target_type, right);
                    }
                }
                self.generate_binary_operator(op, left, right)
            }
            Expr::UnaryExp(op, obj) => self.unaryop_mapping(op, obj),
            Expr::Assign(left, right) => {
                let value = self.generate_expr(right);
                match left.as_ref() {
                    Expr::Identifier(name) => {
                        let slot = self.variables.get(name).unwrap();
                        self.builder.ins().stack_store(value, *slot, 0);
                        value
                    }
                    _ => panic!("Invalid assignment target."),
                }
            }
            Expr::CallFunction(function_anme, args) => {
                let args: Vec<Value> = args.iter().map(|arg| self.generate_expr(arg)).collect();
                let function_ref = self.get_function_reference(function_anme);
                let call = self.builder.ins().call(function_ref, &args);
                self.builder.inst_results(call)[0]
            }
            Expr::Index(array,index ) => {
                let arr = self.generate_expr(array);
                let index = self.generate_expr(index);
                let element_size = self.builder.ins().iconst(types::I32, 4);   // 按照大小为4计算(int)
                let offset = self.builder.ins().imul(index, element_size);
                let addr = self.builder.ins().iadd(arr, offset);
                self.builder.ins().load(types::I32, MemFlags::new(), addr, 0)
            }
            Expr::Member(_obj, _member) | Expr::PointerMember(_obj, _member) => {
                panic!("暂不支持：没有布局信息");
            }
            Expr::Ternary(condition, then_expr, else_expr) => {
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let merge_block = self.builder.create_block();
                let condition_value = self.generate_expr(condition);
                self.builder.ins().brif(condition_value,
                     then_block, &[],
                     else_block, &[]);
                // then
                self.builder.switch_to_block(then_block);
                let then_value = self.generate_expr(then_expr);
                self.builder.ins().jump(merge_block, &[then_value]);
                self.builder.seal_block(then_block);

                // else
                self.builder.switch_to_block(else_block);
                let else_value = self.generate_expr(else_expr);
                self.builder.ins().jump(merge_block, &[else_value]);
                self.builder.seal_block(else_block);

                // merge
                self.builder.switch_to_block(merge_block);
                self.builder.append_block_param(merge_block, types::I32);
                let result = self.builder.block_params(merge_block)[0];
                self.builder.seal_block(merge_block);
                result
            }
            Expr::SizeOf(typ) => {
                let size = sizeof(typ);
                self.builder.ins().iconst(types::I32, size as i64)
            }
            Expr::Char(ch) => {
                self.builder.ins().iconst(types::I8, *ch as i64)
            }
            Expr::String(string) => {
                let data_id = self.module.declare_anonymous_data(false, false).unwrap();
                let mut data = cranelift_module::DataDescription::new();
                data.define(string.as_bytes().to_vec().into_boxed_slice());
                self.module.define_data(data_id, &data).unwrap();

                let global = self.module.declare_data_in_func(data_id, self.builder.func);
                self.builder.ins().global_value(types::I64, global)
            }
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
            // _ => todo!("更多的即将实现。。。（剩下5个位运算）"),
            BinaryOp::BitAnd => inst.band(left, right),
            BinaryOp::BitOr => inst.bor(left, right),
            BinaryOp::BitXor => inst.bxor(left, right),
            BinaryOp::Shl => inst.ishl(left, right),
            BinaryOp::Shr => inst.sshr(left, right),
        }
    }

    fn get_function_reference(&mut self, name: &str) -> FuncRef {
        let function_id = self
            .function_identifiers
            .get(name)
            .copied()
            .expect(&format!("Undefined Function: {}", name));
        self.module
            .declare_func_in_func(function_id, &mut self.builder.func)
    }
    fn unaryop_mapping(&mut self, op: &UnaryOp, obj: &Box<Expr>) -> Value {
        let value = self.generate_expr(obj);
        match op {
            UnaryOp::Neg => self.builder.ins().ineg(value), // 浮点数应该使用fneg 而不是ineg，但是拿类型来判断太麻烦了。
            UnaryOp::Not => {
                // 如果是零，返回一；不是零，返回零。
                // value == 0 ? 1 : 0
                let zero = self.builder.ins().iconst(types::I32, 0);
                self.builder.ins().icmp(IntCC::Equal, value, zero)
            }
            UnaryOp::BitNot => self.builder.ins().bnot(value),
            UnaryOp::Deref => {
                // *ptr。按照I64加载。此处再去分析类型有点麻烦。
                self.builder
                    .ins()
                    .load(types::I64, MemFlags::new(), value, 0)
            }
            UnaryOp::AddressOf => {
                // &var。常见局部变量。仅支持局部变量。
                match obj.as_ref() {
                    Expr::Identifier(id_name) => {
                        let slot = self.variables.get(id_name).unwrap();
                        self.builder.ins().stack_addr(types::I64, *slot, 0)
                        // 此处的I64也应该拿源类型判断。
                    }
                    _ => panic!("Error to take the AdderssOf({:#?})", obj),
                }
            }
            UnaryOp::PreInc => {
                match obj.as_ref() {
                    Expr::Identifier(name) => {
                        let slot = self.variables.get(name).unwrap();
                        let old_value = self.builder.ins().stack_load(types::I32, *slot, 0);
                        let one = self.builder.ins().iconst(types::I32, 1); //拿个数字一出来
                        let new_value = self.builder.ins().iadd(old_value, one);
                        self.builder.ins().stack_store(new_value, *slot, 0);
                        new_value
                    }
                    _ => panic!("Error to increase at VALUE( {:#?} )", obj),
                }
            }
            UnaryOp::PostInc => {
                match obj.as_ref() {
                    Expr::Identifier(name) => {
                        let slot = self.variables.get(name).unwrap();
                        let old_value = self.builder.ins().stack_load(types::I32, *slot, 0);
                        let one = self.builder.ins().iconst(types::I32, 1); //拿个数字一出来
                        let new_value = self.builder.ins().iadd(old_value, one);
                        self.builder.ins().stack_store(new_value, *slot, 0);
                        old_value
                    }
                    _ => panic!("Error to increase at VALUE( {:#?} )", obj),
                }
            }
            UnaryOp::PreDec => {
                match obj.as_ref() {
                    Expr::Identifier(name) => {
                        let slot = self.variables.get(name).unwrap();
                        let old_value = self.builder.ins().stack_load(types::I32, *slot, 0);
                        let one = self.builder.ins().iconst(types::I32, 1); //拿个数字一出来
                        let new_value = self.builder.ins().isub(old_value, one);
                        self.builder.ins().stack_store(new_value, *slot, 0);
                        new_value
                    }
                    _ => panic!("Error to increase at VALUE( {:#?} )", obj),
                }
            }
            UnaryOp::PostDec => {
                match obj.as_ref() {
                    Expr::Identifier(name) => {
                        let slot = self.variables.get(name).unwrap();
                        let old_value = self.builder.ins().stack_load(types::I32, *slot, 0);
                        let one = self.builder.ins().iconst(types::I32, 1); //拿个数字一出来
                        let new_value = self.builder.ins().isub(old_value, one);
                        self.builder.ins().stack_store(new_value, *slot, 0);
                        old_value
                    }
                    _ => panic!("Error to increase at VALUE( {:#?} )", obj),
                }
            }
        }
    }
}

/// ## 从cnone::ast::Type和cnone::lexer::types::Type 映射到cranelift::prelude::Type
pub fn clif_type(typ: &ast::Type) -> Type {
    match typ {
        ast::Type::Int | ast::Type::Long => types::I32,
        ast::Type::Float => types::F32,
        ast::Type::Double => types::F64,
        ast::Type::Char => types::I8,
        ast::Type::Void => types::INVALID,
        _ => types::I32,
    }
}

///
/// # 返回类型的字节。
/// # ReturnType usize
fn sizeof(typ: &ast::Type) -> usize {
    match typ {
        ast::Type::Char => 1,
        ast::Type::Short => 2,
        ast::Type::Int | ast::Type::Float => 4,
        ast::Type::Long | ast::Type::Double => 8,
        ast::Type::Pointer(_) => 8,
        ast::Type::Signed | ast::Type::Unsigned | ast::Type::Void => 4,
    }
}
/// ## 应该返回当前变量在cranelift::types里对应的Type。
/// 
/// 
/// # ReturnType cranelift::types::Type
#[allow(unused)]
fn get_type_of() -> types::Type {
    return types::I64;
}