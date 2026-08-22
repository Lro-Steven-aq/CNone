use super::stmt::Stmt;

#[derive(Debug,Clone,PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}