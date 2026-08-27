#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    For,
    While,
    Do,
    Return,
    Break,
    Continue,
    Switch,
    Case,
    Default,
    Goto, // Warning!
    Sizeof,

    Struct,
    Typedef,
}
