use std::collections::HashMap;
use crate::ir::variable_slot_table::{Index, Slot, VariableTable};
use crate::ir::value::Value;
use crate::ir::types::Type;

#[derive(Debug, Clone)]
pub struct VariablesSlotTable {
    slots: VariableTable,
}
impl VariablesSlotTable {
    pub fn new() -> Self {
        Self { 
            slots: VariableTable { 
                variables: HashMap::new(), 
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
        let slot = Slot { value };
        self.slots.slots.insert(index, slot);
    }
}

#[test]
fn test_variable_slot_table() {
    let mut table = VariablesSlotTable::new();
    table.set_variable(0, Value::new(Type::INT));
    println!("### {:#?}", table);
    let mut m = table.get_variable(0);
    println!("m: {:#?}", m);
    m.set_value(90, None);
    table.set_variable(0, m);
    println!("### {:#?}", table);
}