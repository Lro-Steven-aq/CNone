use std::{fmt::{Display, write}, sync::OnceLock};

use crate::ir::{types::Type, value::Value, variable_slot_table::Index};


const ERROR_MESSAGE_NOT_ENOUGH: &str = "Arguments numbers doesn't seem to be enough";
#[derive(Debug)]
pub struct OperationStack {
    pub stack: Vec<Value>,
}

impl OperationStack {
    pub fn new () -> Self {
        Self { stack: Vec::new() }
    }
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or_else(||{
                eprintln!("The stack is Empty");
                Value::default()
            }
        )
    }
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    pub fn add(&mut self) {
        let right = self.stack.pop().expect(ERROR_MESSAGE_NOT_ENOUGH);
        let left = self.stack.pop().expect(ERROR_MESSAGE_NOT_ENOUGH);
        self.stack.push(left + right);
    }

    pub fn sub(&mut self) {
        let right = self.stack.pop().expect(ERROR_MESSAGE_NOT_ENOUGH);
        let left = self.stack.pop().expect(ERROR_MESSAGE_NOT_ENOUGH);
        self.stack.push(left - right);
    }

}

impl Display for OperationStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for i in self.stack.clone() {
            write!(f, "{}", i).unwrap();
            write!(f, ", ").unwrap();
        }
        write!(f, "]").unwrap();
        std::fmt::Result::Ok(())
    }
}
impl Default for OperationStack {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_operation_stack() {
    let mut stack = OperationStack::new();
    let mut l = Value::new(Type::CHAR);
    l.set_value(66, None, None);
    let mut r = Value::new(Type::FLOAT);
    r.set_value(88, Some(7011), Some(4));
    stack.push(l);
    stack.push(r);
    println!("{}", stack);
    stack.add();
    println!("{}", stack);

}