use std::collections::HashMap;

use crate::asm::{Opcode, ProcessDefinition, ProcessType};
use crate::vm::InstructionProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct Rom {
    processes: HashMap<ProcessType, ProcessDefinition>,
}

impl Rom {
    pub fn new(processes: HashMap<ProcessType, ProcessDefinition>) -> Self {
        Self { processes }
    }
}

impl InstructionProvider for Rom {
    fn instructions(&self) -> &[Opcode] {
        // TODO: return instructions for given process type
        self.processes.get(&ProcessType(0)).unwrap().instructions()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cartridge {
    title: String,
    rom: Rom,
}

impl Cartridge {
    pub fn new(title: &str, rom: Rom) -> Self {
        Self {
            title: title.to_string(),
            rom,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }
}
