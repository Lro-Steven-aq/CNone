

#[derive(Debug,Clone,PartialEq)]
pub enum Operator {
    //+ - * / % 
    Plus, Minus, Star, Slash, Percent,

    //++ --
    PlusPlus, MinusMinus,

    // 赋值 = ，及其变种。(+= -= *= /= %=)
    Assign, PlusAssign, MinusAssign, StarAssign, SlashAssign, PercentAssign,XorAssign

    //compare >, <, !=, >=, <=, ==
    Eq, Neq, Lt, Gt, Le, Ge,

    //Logic && || !
    LogicAnd, LogicOr, LogicNot,

    //bit & | ^ ~ << >>
    And, Or, Xor, Tilde, Shl, Shr,

    //other:剩余的：结构体访问成员，结构体指针访问成员。
    Arrow, Dot,
}
