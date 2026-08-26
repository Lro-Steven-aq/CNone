mod code_generator;
mod loop_info;
mod struct_info;

use std::collections::HashMap;

use cranelift::codegen::ir::UserFuncName;
use cranelift::codegen::settings;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::ObjectBuilder;
use cranelift_object::ObjectModule;
use target_lexicon::triple;

use crate::ast;
use crate::ast::declarations::Decl;
use crate::ast::program::Program;
use crate::ast::stmt::Stmt;

use code_generator::CodeGenerator;
use code_generator::clif_type;

/// ## 用来编译程序到目标文件。
/// ### 缺失链接。
/// # Input cnone::ast::program::Program
/// # Return Vec<u8>
pub fn compile_program_to_bytecode(program: &Program) -> Vec<u8> {
    let triple = triple!("x86_64-unknown-linux-gnu");
    let flag_builder = settings::builder();

    let isa_builder = isa::lookup(triple).unwrap();
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();
    let builder =
        ObjectBuilder::new(isa, "main", cranelift_module::default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);
    // /////////////////////////////////////////////////////////
    //        声明所有的函数，然后给予其定义，再，，，              ///
    // /////////////////////////////////////////////////////////
    let mut function_identifiers = HashMap::new();
    for decl in &program.decls {
        if let Decl::Function(function) = decl {
            let mut signature = module.make_signature();

            if function.return_type != ast::Type::Void {
                signature
                    .returns
                    .push(AbiParam::new(clif_type(&function.return_type)));
            }
            for param in &function.params {
                signature.params.push(AbiParam::new(clif_type(&param.typ)));
            }
            let function_identifier = module
                .declare_function(&function.name, Linkage::Export, &signature)
                .unwrap();
            function_identifiers.insert(function.name.clone(), function_identifier);
        }
    }
    // /////////////////////////////////////////////////////////////
    // ///////////            2.函数体.                //////////////
    // /////////////////////////////////////////////////////////////
    let mut context = module.make_context();
    let mut function_builder_context = FunctionBuilderContext::new();
    for decl in &program.decls {
        if let Decl::Function(function) = decl {
            let function_identifier = function_identifiers.get(&function.name).expect("");

            context.func.signature = module
                .declarations()
                .get_function_decl(*function_identifier)
                .signature
                .clone();
            context.func.name = UserFuncName::user(0, function_identifier.as_u32());

            let mut builder =
                FunctionBuilder::new(&mut context.func, &mut function_builder_context);
            let entry_block = builder.create_block();

            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            let mut code_generator = CodeGenerator {
                module: &mut module,
                builder: builder,
                variables: HashMap::new(),
                variable_types: HashMap::new(),
                function_identifiers: function_identifiers.clone(),
                current_block_terminated: false,
                loop_stack: Vec::new(),
                structs: HashMap::new(),
            };
            // /////////////////////////////////////////////////////////
            // ///////              参数绑定与variables。          ///////
            // /////////////////////////////////////////////////////////
            for (index, param) in function.params.iter().enumerate() {
                let typ = clif_type(&param.typ);
                let slot = code_generator.declare_variable(&param.name, typ);
                let mut value = code_generator.builder.block_params(entry_block)[index];
                // ///
                let value_type = code_generator.builder.func.dfg.value_type(value);
                if value_type != typ {
                    if value_type.bytes() > typ.bytes() {
                        value = code_generator.builder.ins().ireduce(typ, value); // 缩减
                    } else {
                        value = code_generator.builder.ins().sextend(typ, value); // 扩大
                    }
                }
                code_generator.builder.ins().stack_store(value, slot, 0);
            }
            if let Some(body) = &function.body {
                code_generator.generate_stmt(&Stmt::Block(body.clone()));
            }
            if !code_generator.is_block_filled() {
                code_generator.builder.ins().return_(&[]);
            }

            code_generator.builder.finalize();
            module
                .define_function(*function_identifier, &mut context)
                .unwrap();
            context.clear();
        }
    }
    module.finish().emit().unwrap()
}
