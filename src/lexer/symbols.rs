/**
 * # 那些有特殊意义的符号。比如：
 *      ()[]{};,:
 * 特别的，引号是用来标记字符串的，不予做记录。 
 */
#[derive(Debug,Clone,PartialEq)]
pub enum Symbol{
    // (        )       {       }
    LParen, RParen, LBrace, RBrace, 
    //   [        ]        ;        ,       :
    LBracket, RBracket, Semicolon, Comma, Colon,
}