use std::collections::HashMap;
use std::rc::Rc;

use crate::asm::{Instruction, ProcessDefinition, ProcessType};
use crate::error::{Result, RuntimeError};
use crate::vm::RomProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct Rom {
    processes: HashMap<ProcessType, ProcessDefinition>,
}

impl Rom {
    pub fn new(processes: HashMap<ProcessType, ProcessDefinition>) -> Self {
        Self { processes }
    }
}

impl RomProvider for Rom {
    fn instructions(&self, process_type: ProcessType) -> Result<Rc<[Instruction]>> {
        let process = self
            .processes
            .get(&process_type)
            .ok_or(RuntimeError::ProcessNotFound(process_type))?;
        Ok(process.instructions())
    }

    fn args(&self, process_type: ProcessType) -> Result<Rc<[usize]>> {
        let process = self
            .processes
            .get(&process_type)
            .ok_or(RuntimeError::ProcessNotFound(process_type))?;
        Ok(process.args())
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
