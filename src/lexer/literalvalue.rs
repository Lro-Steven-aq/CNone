
/**
 * 用来表示常量。
 * 变量的表示方法类似。
 */
#[derive(Debug,Clone,PartialEq)]
pub enum Literal {
    Interage(i64),
    Float(f64),
    Char(char),
    String(String),
}
