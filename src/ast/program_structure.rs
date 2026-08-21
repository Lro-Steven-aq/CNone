use crate::lexer::types::Type;

#[derive(Debug,Clone,PartialEq)]

pub struct Program {
    pub decls: Vec<Decl>,      /*写getter,setter还是麻烦。 */
}

#[derive(Debug,Clone,PartialEq)]

pub enum Decl {             // 总声明，包括了函数和变量。
    Function(FunctionDecl),
    Varible(VaribleDecl),
}

#[derive(Debug,Clone,PartialEq)]

pub struct FunctionDecl {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Block>,
}


#[derive(Debug,Clone,PartialEq)]
pub struct VaribleDecl {
    pub typ: Type,
    pub name: String,
    pub init: Option<Expr>,
}

#[derive(Debug,Clone,PartialEq)]

pub struct Param {
    pub typ: Type,
    pub name: String,
}

#[derive(Debug,Clone,PartialEq)]

pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug,Clone,PartialEq)]

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

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Identifier(String),
    BinaryExp(BinaryOp, Box<Expr>, Box<Expr>),  //expr1 binop expr2
    UnaryExp(UnaryOp, Box<Expr>),     //unaryop expr
    Assign(Box<Expr>, Box<Expr>),
    CallFunction(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, String),
    PointerMember(Box<Expr>, String),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),   //三元运算符
    SizeOf(Type),
}

#[derive(Debug,Clone,PartialEq)]

pub enum BinaryOp {
    // +  -    *    /    %
    Add, Sub, Mul, Div, Mod,
    // ==  != <  <=  >  >=
    Eq, Ne, Lt, Le, Gt, Ge,
    // &&  ||
    And, Or,
    // &      |      ^      <<   >>
    BitAnd, BitOr, BitXor, Shl, Shr,
}

#[derive(Debug,Clone,PartialEq)]

pub enum UnaryOp {
    Neg,   // 负号
    Not,
    BitNot,
    Deref,
    AddressOf,
    PreInc, PostInc,
    PreDec, PostDec,
}