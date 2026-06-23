use std::collections::HashMap;

use crate::asm::{Location, ProcessId, Value};
use crate::vm::{Memory, Result, RuntimeError};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Locals {
    locals: HashMap<ProcessId, Vec<Value>>,
}

impl Locals {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
        }
    }
}

impl Memory for Locals {
    fn poke(&mut self, id: ProcessId, addr: usize, value: Value) -> Result<()> {
        let entry = self
            .locals
            .get_mut(&id)
            .ok_or(RuntimeError::LocalNotFound(Location::default(), id, addr))?;
        *entry
            .get_mut(addr)
            .ok_or(RuntimeError::LocalNotFound(Location::default(), id, addr))? = value;
        Ok(())
    }

    fn peek(&self, id: ProcessId, addr: usize) -> Result<Value> {
        let entry = self
            .locals
            .get(&id)
            .ok_or(RuntimeError::LocalNotFound(Location::default(), id, addr))?;
        entry
            .get(addr)
            .copied()
            .ok_or(RuntimeError::LocalNotFound(Location::default(), id, addr))
    }
}