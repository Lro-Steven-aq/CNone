use cranelift::codegen::ir::Block;

/// # 用来记录循环
/// 作为CodeGenerator的依赖。
#[derive(Clone, Copy)]
pub struct LoopInfo {
    pub exit_block: Block,
    pub continue_block: Block,
}
