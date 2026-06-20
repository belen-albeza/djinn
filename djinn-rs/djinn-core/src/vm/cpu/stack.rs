use crate::asm::Value;

pub struct Stack {
    items: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.items.pop()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
