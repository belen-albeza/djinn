use crate::asm::{Location, Value};
use crate::error::{Result, RuntimeError};
use crate::vm::ValueStack;

#[derive(Debug)]
pub struct Stack {
    items: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl ValueStack for Stack {
    fn push(&mut self, value: Value) {
        self.items.push(value);
    }

    fn pop(&mut self, location: Location) -> Result<Value> {
        let value = self
            .items
            .pop()
            .ok_or(RuntimeError::StackUnderflow(location))?;
        Ok(value)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
