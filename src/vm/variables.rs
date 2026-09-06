//! 
//! 变量表。
//! 其成员slots: VariableTable是其具体实现
//! 

use std::collections::HashMap;
use std::fmt::Display;
use crate::ir::variable_slot_table::{Index, Slot, VariableTable};
use crate::ir::value::Value;
use crate::ir::types::Type;
use crate::vm::stack::OperationStack;

#[derive(Debug, Clone, PartialEq)]
pub struct VariablesSlotTable {
    slots: VariableTable,
}
impl VariablesSlotTable {
    pub fn new() -> Self {
        Self { 
            slots: VariableTable { 
                slots: HashMap::new() 
            } 
        }
    }
    pub fn get_variable(&mut self, index: Index) -> Value {
        let slot = self.slots.slots.get(&index).unwrap();
        slot.value
    }
    pub fn set_variable(&mut self, index: Index, value: Value) {
        // let __value = Value::new(Type::CHAR);
        // let typ = value.get_type();
        // __value.set_type(typ);
        let slot = Slot { value, index: index };
        self.slots.slots.insert(index, slot);
    }

    fn remove_variable(&mut self, index: Index) {
        self.slots.slots.remove(&index);
    }

    pub fn load(&mut self, stack:&mut OperationStack, index: Index) -> Value {
        let value = self.get_variable(index);
        stack.push(value);
        // remove it from variable table.
        self.remove_variable(index);
        value
    }

    pub fn store(&mut self, stack:&mut OperationStack, index: Index) {
        let value = stack.pop();
        self.set_variable(index, value);
    }
}

impl Default for VariablesSlotTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for VariablesSlotTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.slots.slots)
    }
}

#[test]
fn test_variable_slot_table() {
    /* for 循环（计数循环）
        PUSH CHAR 65
        STORE 0
        PUSH INT 0
        STORE 1
        PUSH 10
        STORE 2
        
    for_loop_body:
        /*
            BODY IS HERE...
         */
        LOAD 1
        LOAD 2
        LT
        JUMPIF if_true
        JUMP endif
    endif:
        CALL ECHO
    if_true:
        STORE 2
        INC
        STORE 1
        JUMPIF for_loop_body
    
     */
    let mut stack = OperationStack::default();
    let mut table = VariablesSlotTable::new();
    // init
    let mut i = Value::new(Type::INT);
    i.set_value(0, None, None);
    let mut max = Value::new(Type::INT);
    max.set_value(10, None, None);

    table.set_variable(0, i);
    table.set_variable(1, max);

    table.load(&mut stack, 0);
    table.load(&mut stack, 1);
    stack.gt();
    println!("{}\n{}", stack, table);
}