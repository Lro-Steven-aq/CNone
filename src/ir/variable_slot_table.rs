use std::collections::HashMap;
use crate::ir::value::Value;


pub type Index = usize;
#[derive(Debug, Clone, PartialEq)]
pub struct VariableTable {
    // pub variables: HashMap<String, Index>,      // 变量名->ID
    pub slots: HashMap<Index, Slot>,            // ID->Slot Object
}                                               // Slot Object 等于 Value

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub value: Value,
}