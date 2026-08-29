use std::collections::HashMap;
use crate::ir::value::Value;


pub type Index = usize;
#[derive(Debug, Clone, PartialEq)]
pub struct VariableTable {
    pub variables: HashMap<String, Index>,
    pub slots: HashMap<Index, Slot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub value: Value,
}