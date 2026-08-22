

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