use super::block::Block;
use super::declarations::VaribleDecl;
use super::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    VaribleDecl(VaribleDecl),
    Return(Option<Expr>),

    //条件，   为真时执行的代码块， 为假时执行的代码块。
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Option<Expr>, Option<Expr>, Option<Expr>),
    Block(Block),
    Continue,
}
