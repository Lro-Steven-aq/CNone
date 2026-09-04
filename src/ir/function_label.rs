//! ## 这是一个 `For` 循环:
//! ```ignore
//! init:
//!     PUSH INT 0
//!     STORE 0
//!     JUMP __func__
//! func1:
//!     LOAD 0
//!     INC
//!     PUSH INT 10
//!     GE
//!     JUMPIF endif
//!     JUMP __func__
//! endif:
//! __func__：
//!     FUNCTION-BODY
//!     JUMP func1
//! ```
// use super::Stmt;
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLabel {
    name: String,
    // stmts: Vec<Stmt>,  function body
    //body: Body,
}
