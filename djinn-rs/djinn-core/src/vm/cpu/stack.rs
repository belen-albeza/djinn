use crate::asm::Value;

pub struct Stack {
    items: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
