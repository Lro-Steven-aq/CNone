use super::declarations::Decl;

#[derive(Debug,Clone,PartialEq)]
pub struct Program {
    pub decls: Vec<Decl>,      /*写getter,setter还是麻烦。 */
}