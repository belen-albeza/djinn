use std::collections::HashMap;

use crate::asm::{Location, ProcessId, Value};
use crate::vm::{GlobalMemory, LocalMemory, Result, RuntimeError};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Locals {
    locals: HashMap<ProcessId, Vec<Option<Value>>>,
}

impl Locals {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
        }
    }
}

impl LocalMemory for Locals {
    fn poke(&mut self, id: ProcessId, addr: usize, value: Value) -> Result<()> {
        let slots = self.locals.entry(id).or_default();
        if addr >= slots.len() {
            slots.resize(addr + 1, None); // pad earlier slots with None
        }
        slots[addr] = Some(value);
        Ok(())
    }

    fn peek(&self, id: ProcessId, addr: usize) -> Result<Value> {
        self.locals
            .get(&id)
            .and_then(|slots| slots.get(addr).copied().flatten())
            .ok_or(RuntimeError::LocalNotFound(Location::default(), id, addr))
    }

    fn free(&mut self, id: ProcessId) {
        self.locals.remove(&id);
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Globals {
    globals: HashMap<usize, Value>,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }
}

impl GlobalMemory for Globals {
    fn poke(&mut self, addr: usize, value: Value) -> Result<()> {
        self.globals.insert(addr, value);
        Ok(())
    }

    fn peek(&self, addr: usize) -> Result<Value> {
        self.globals
            .get(&addr)
            .copied()
            .ok_or(RuntimeError::GlobalNotFound(Location::default(), addr))
    }
}
